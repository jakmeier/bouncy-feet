<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
  import {
    requestNewGuestSession,
    submitStats,
    submitUserMetadata,
  } from '$lib/stats';
  import { generateRandomUsername } from '$lib/username';
  import { onMount, setContext } from 'svelte';
  import { readable, writable } from 'svelte/store';
  import { showExperimentalFeatures } from '$lib/stores/FeatureSelection.js';
  import { ONBOARDING_STATE } from '$lib/onboarding';
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
      return await requestNewGuestSession().then((response) => {
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
      await submitUserMetadata(key, value);
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
      userLessonProgress: {},
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

  /**
   * @param {import("$lib/instructor/bouncy_instructor").DetectedStep[]} dance
   * @returns {DanceSessionResult?}
   */
  function computeDanceStats(dance) {
    if (dance.length < 1) {
      return null;
    }

    let stats = {};
    let totalSteps = 0;
    let totalExp = 0;
    /**
     * @type {never[]}
     */
    const bpms = [];
    for (const step of dance) {
      if (!step.name.includes('Idle')) {
        totalSteps += 1;
        if (!stats[step.name]) {
          stats[step.name] = {
            slow: 0,
            mid: 0,
            fast: 0,
            veryFast: 0,
          };
        }
        const bpm = step.bpm;
        if (bpm > 130) {
          stats[step.name].veryFast += 1;
        } else if (bpm > 100) {
          stats[step.name].fast += 1;
        } else if (bpm > 50) {
          stats[step.name].mid += 1;
        } else {
          stats[step.name].slow += 1;
        }
      }
    }

    const duration = Number(dance[dance.length - 1].end - dance[0].start);

    for (let [key, stat] of Object.entries(stats)) {
      const stepGainedExp =
        stat.slow * 10 + stat.mid * 15 + stat.fast * 20 + stat.veryFast * 25;
      totalExp += stepGainedExp;
    }

    return {
      numSteps: totalSteps,
      experience: totalExp,
      duration,
      stats,
      bpms,
    };
  }

  /**
   * @param {DanceSessionResult} result
   */
  function addDanceToStats(result) {
    $user.recordedDances += 1;
    $user.recordedSteps += result.numSteps;
    $user.recordedSeconds += result.duration / 1000;
    for (let [key, stat] of Object.entries(result.stats)) {
      if (!$user.userSteps[key]) {
        $user.userSteps[key] = {
          experience: 0,
          count: 0,
        };
      }
      const numSteps = stat.slow + stat.mid + stat.fast + stat.veryFast;
      $user.userSteps[key].count = $user.userSteps[key].count + numSteps;
      const stepGainedExp =
        stat.slow * 10 + stat.mid * 15 + stat.fast * 20 + stat.veryFast * 25;
      $user.userSteps[key].experience =
        $user.userSteps[key].count + stepGainedExp;
    }

    // trigger subscribers
    $user = $user;

    try {
      submitStats($user);
    } catch (err) {
      console.warn('Submitting stats failed', err);
    }
  }

  /**
   * @param {string} courseId
   * @param {number} lessonIndex
   * @param {number} level
   */
  function recordFinishedLesson(courseId, lessonIndex, level) {
    if (!$user.userLessonProgress[courseId]) {
      $user.userLessonProgress[courseId] = {
        lessons: {},
      };
    }
    const before = $user.userLessonProgress[courseId].lessons[lessonIndex] || 0;
    const newLevel = Math.max(before, level);
    $user.userLessonProgress[courseId].lessons[lessonIndex] = newLevel;
    // trigger subscribers
    $user = $user;
  }

  /** @type {UserContextData} */
  const userCtx = {
    store: user,
    clientSession,
    setUserMeta,
    computeDanceStats,
    addDanceToStats,
    recordFinishedLesson,
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
