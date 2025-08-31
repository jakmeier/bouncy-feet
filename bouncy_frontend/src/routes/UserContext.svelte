<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
  import { requestNewGuestSession, apiRequest } from '$lib/stats';
  import { generateRandomUsername } from '$lib/username';
  import { onMount, setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import { showExperimentalFeatures } from '$lib/stores/FeatureSelection.js';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { DetectionResult } from 'bouncy_instructor';
  import { initKeycloakAuth } from '$lib/keycloak';
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

  // Pwa auth works independent of the api server, generating a token to be used
  // for PeerTube only (for now).
  /** @type {PwaAuth} */
  const pwaAuth = $state({
    isAuthenticated: false,
    keycloakInstance: null,
    userProfile: null,
    peerTubeToken: null,
  });

  if (browser) {
    initKeycloakAuth(pwaAuth);
  }

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

  function authHeader() {
    // TODO: Switch to OAuth2 for registered users
    return clientSession.id
      ? {
          Authorization: `ClientSession ${clientSession.id}:${clientSession.secret}`,
        }
      : {};
  }

  /**
   * @param {string} path
   * @returns {Promise<Response | null | undefined>}
   */
  async function authenticatedGet(path) {
    return await authenticatedApiRequest('GET', path, {}, undefined);
  }

  /**
   * @param {string} method
   * @param {string} path
   * @param {object} headers
   * @param {string|undefined} body
   */
  async function authenticatedApiRequest(method, path, headers, body) {
    let auth = authHeader();
    if (!auth.Authorization) {
      // This can happen when code calls this before clientSession is loaded
      throw new Error('Authenticated API call without auth header');
    }
    const options = {
      method,
      headers: {
        ...headers,
        ...auth,
      },
      body,
    };

    try {
      return await apiRequest(path, options);
    } catch (errResponse) {
      if (
        (errResponse && errResponse.status === 401) ||
        errResponse.status == 403
      ) {
        // <Temporary code>
        // Some client sessions have been lost. They need to be replaced.

        // try to get a new session
        const newSession = await requestNewGuestSession();

        // replace local session id and secret but keep other fields
        console.warn('replacing old session with a new');
        localStorage.clientSessionId = newSession.client_session_id;
        localStorage.clientSessionSecret = newSession.client_session_secret;
        clientSession.id = newSession.client_session_id;
        clientSession.secret = newSession.client_session_secret;

        // now update the server about our local state
        for (const [key, value] of Object.entries(clientSession.meta)) {
          if (typeof key === 'string' && typeof value === 'string') {
            setUserMeta(key, value);
          }
        }

        // now we can try to make the unauthorized request again, with new auth headers
        let auth = authHeader();
        const options = {
          method,
          headers: {
            ...headers,
            ...auth,
          },
          body,
        };
        return await apiRequest(path, options);
      }
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

      const headers = {
        'Content-Type': 'application/json',
      };
      const body = JSON.stringify({
        key_name: key,
        key_value: value,
        // chrono can parse the time including the timezone from this
        last_modified: new Date().toISOString(),
        // the only existing version for now
        version: 0,
      });

      return await authenticatedApiRequest(
        'POST',
        '/user/meta/update',
        headers,
        body
      );
    }
  }

  /**
   * @param {string} activityId
   * @param {DanceSessionResult} sessionResult
   */
  async function submitActivityCall(activityId, sessionResult) {
    const headers = {
      'Content-Type': 'application/json',
    };
    const body = JSON.stringify({
      client_session_id: Number.parseInt(clientSession.id),
      client_session_secret: clientSession.secret,

      activity_id: activityId,
      recorded_at: sessionResult.timestamp.toISOString(),
      hits: sessionResult.hits,
      misses: sessionResult.misses,
      steps: sessionResult.numSteps,
    });

    return await authenticatedApiRequest(
      'POST',
      '/new_guest_activity',
      headers,
      body
    );
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
      // TODO: get this from Keycloak after the user is created
      // TODO: set this from the API server if it has it (and compare on login )
      openid: undefined,
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

    try {
      // not awaiting here, it can happen in the background
      submitActivityCall(limitedId, sessionResult);
    } catch (err) {
      console.warn('Submitting activity stats failed', err);
    }
    // TODO: store locally

    return sessionResult;
  }

  function loggedInToKeycloak() {
    return pwaAuth.keycloakInstance?.authenticated || false;
  }

  async function refreshUserId() {
    try {
      const response = await authenticatedGet('/user');
      const userInfo = await response?.json();
      if (!userInfo || userInfo.sub === undefined) {
        throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
      }

      // note: Sub may be null, this just means this is a guest user OR the user is not logged in.
      // Eitherway, if there is already a known user.openid, it should not be overwritten.
      if (userInfo.sub) {
        $user.openid = userInfo.sub;
      }

      // trigger subscribers?
      $user = $user;
    } catch (errResponse) {
      console.warn('Failed reading user info');
    }
  }

  /** @type {UserContextData} */
  const userCtx = {
    store: user,
    clientSession,
    pwaAuth,
    loggedInToKeycloak,
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
    } else {
      if (!$user.openid) {
        await refreshUserId();
      }
    }
    has.mounted = true;
  });
</script>

{#if has.mounted}
  {@render children?.()}
{/if}
