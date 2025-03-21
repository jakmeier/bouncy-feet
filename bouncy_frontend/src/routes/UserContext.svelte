<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
  import { requestNewGuestSession, submitStats, apiRequest } from '$lib/stats';
  import { generateRandomUsername } from '$lib/username';
  import { onMount, setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import { showExperimentalFeatures } from '$lib/stores/FeatureSelection.js';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { DetectionResult } from 'bouncy_instructor';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /**
   * Client sessions are available for registered users and guests.
   *
   * There can be multiple client sessions per user, when they use multiple
   * devices, browsers, or browser profiles.
   *
   * To create a new guest session, the client has to make a request to the API server.
   * This could be changed in the future, to allow offline initialization.
   *
   * TODO: registered user client sessions
   */

  // This state is read-only. Updates must go through setters. Otherwise, they
  // are not persisted.
  // Actually initialized in onMount
  /** @type {ClientSession} */
  const clientSession = $state({});

  /**
   * Load from localStorage or create a new client session through the API.
   *
   * @return {Promise<ClientSession>}
   */
  async function loadClientSessionAsync() {
    if (!browser) {
      return {
        id: '',
        secret: '',
        meta: {},
      };
    }
    if (localStorage.clientSessionId) {
      return {
        id: localStorage.clientSessionId,
        secret: localStorage.clientSessionSecret,
        meta: parseOrNull(localStorage.userMeta) || {},
      };
    } else {
      return await requestNewGuestSession()
        .then((response) => {
          if (response.client_session_id && response.client_session_secret) {
            /** @type {ClientSession} */
            const newClientSession = {
              id: response.client_session_id,
              secret: response.client_session_secret,
              meta: {
                onboarding: ONBOARDING_STATE.FIRST_VISIT,
              },
            };
            localStorage.clientSessionId = newClientSession.id;
            localStorage.clientSessionSecret = newClientSession.secret;
            localStorage.userMeta = JSON.stringify(newClientSession.meta);
            return newClientSession;
          } else {
            console.error(
              'Failed to create a guest session. Response:',
              response
            );
            return {
              id: '',
              secret: '',
              meta: {},
            };
          }
        })
        .catch((err) => {
          console.error('Failed to create a guest session. Error:', err);
          return {
            id: localStorage.clientSessionId,
            secret: localStorage.clientSessionSecret,
            meta: parseOrNull(localStorage.userMeta) || {
              onboarding: ONBOARDING_STATE.FIRST_VISIT,
            },
          };
        });
    }
  }

  /**
   * @param {string} key
   * @param {string} value
   */
  async function setUserMeta(key, value) {
    if (key) {
      // First persist in localStorage
      const meta = JSON.parse(localStorage.getItem('userMeta') || '{}');
      meta[key] = value;
      localStorage.setItem('userMeta', JSON.stringify(meta));

      // Now also update client state
      // @ts-ignore
      clientSession.meta[key] = value;

      // sync changes to API backend
      // TODO: Switch to OAuth2 for registered users
      let auth = clientSession.id
        ? {
            Authorization: `ClientSession ${clientSession.id}:${clientSession.secret}`,
          }
        : {};
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...auth,
        },
        body: JSON.stringify({
          key_name: key,
          key_value: value,
          // chrono can parse the time including the timezone from this
          last_modified: new Date().toISOString(),
          // the only existing version for now
          version: 0,
        }),
      };

      return await apiRequest('/user/meta/update', options);
    }
  }

  /**
   * @param {string} key
   */
  function parseOrNull(key) {
    try {
      return JSON.parse(key);
    } catch (e) {
      return null;
    }
  }

  // TODO: user data should be synced with the server
  const stored = browser ? parseOrNull(localStorage.user) : null;
  if (stored && !stored.id) {
    stored.id = crypto.randomUUID();
  }
  if (stored && !stored.userSteps) {
    stored.userSteps = {};
  }
  if (stored && !stored.userLessonProgress) {
    stored.userLessonProgress = {};
  }
  if (stored && stored.consentSendingStats === undefined) {
    stored.consentSendingStats = false;
  }
  if (stored && stored.experimentalFeatures === undefined) {
    stored.experimentalFeatures = false;
  }

  // TODO: consider making this $state() instead of writable
  /** @type {import('svelte/store').Writable<UserData>} */
  const user = writable(
    stored || {
      id: crypto.randomUUID(),
      publicName: generateRandomUsername(),
      score: 0,
      recordedDances: 0,
      recordedSeconds: 0,
      recordedSteps: 0,
      userSteps: {},
      consentSendingStats: false,
      experimentalFeatures: false,
    }
  );
  if (browser) {
    user.subscribe((value) => (localStorage.user = JSON.stringify(value)));
    user.subscribe((value) => {
      showExperimentalFeatures(value.experimentalFeatures);
    });
  }

  // Update local stats, offline only.
  /**
   * @param {DanceSessionResult} result
   */
  function addDanceToStats(result) {
    $user.recordedDances += 1;
    $user.recordedSteps += result.numSteps;
    $user.recordedSeconds += result.duration / 1000;

    // trigger subscribers
    $user = $user;
  }

  /**
   * @param {string} courseId
   * @param {number} lessonIndex
   * @param {DetectionResult} detection
   *
   * @returns {DanceSessionResult?}
   */
  function submitCourseLesson(courseId, lessonIndex, detection) {
    const id = `course/${courseId}[${lessonIndex}]`;
    return submitActivity(id, detection);
  }

  /**
   * @param {string} warmupId
   * @param {DetectionResult} detection
   *
   * @returns {DanceSessionResult?}
   */
  function submitWarmup(warmupId, detection) {
    const id = `warmup/${warmupId}`;
    return submitActivity(id, detection);
  }

  /**
   * @param {string} stepId
   * @param {number} bpm
   * @param {DetectionResult} detection
   *
   * @returns {DanceSessionResult?}
   */
  function submitStepTraining(stepId, bpm, detection) {
    const id = `step/${stepId}[${bpm}]`;
    return submitActivity(id, detection);
  }

  /**
   * @param {string} activityId
   * @param {DetectionResult} detection
   *
   * @returns {DanceSessionResult?}
   */
  function submitActivity(activityId, detection) {
    const limitedId = activityId.slice(0, 256);
    const hits = detection.poseMatches;
    const misses = detection.poseMisses;
    const steps = detection.steps();
    const numSteps = steps.length;
    const duration =
      steps.length > 0
        ? Number(steps[steps.length - 1].end - steps[0].start)
        : 0;

    /** @type {DanceSessionResult} */
    const sessionResult = {
      numSteps,
      hits,
      misses,
      duration,
      timestamp: new Date(),
    };

    // TODO: actually submit something
    // try {
    //   submitStats($user);sessionResult,limitedId
    // } catch (err) {
    //   console.warn('Submitting stats failed', err);
    // }
    // TODO: store locally if it failed

    return sessionResult;
  }

  /** @type {UserContextData} */
  const userCtx = {
    store: user,
    clientSession,
    setUserMeta,
    submitWarmup,
    submitCourseLesson,
    submitStepTraining,
    addDanceToStats,
  };
  setContext('user', userCtx);

  const has = $state({ mounted: false });
  onMount(async () => {
    // ensure the store is initialized

    const actualClientSession = await loadClientSessionAsync();
    Object.assign(clientSession, actualClientSession);

    if (!clientSession.id) {
      console.warn('No client session');
    }
    has.mounted = true;
  });
</script>

{#if has.mounted}
  {@render children?.()}
{/if}
