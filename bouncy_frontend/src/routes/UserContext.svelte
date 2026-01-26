<script>
  /**
   * Provides access to a user local storage
   */
  import { browser } from '$app/environment';
  import { requestNewGuestSession, apiRequest, API_ERROR } from '$lib/stats';
  import { generateRandomUsername } from '$lib/username';
  import { onMount, setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import { showExperimentalFeatures } from '$lib/stores/FeatureSelection.js';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { DetectionResult } from 'bouncy_instructor';
  import { client as peerTubeApi } from '$lib/peertube-openapi/client.gen';
  import { PUBLIC_API_BASE, PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { fetchPeerTubeUser } from '$lib/peertube';
  import { KvSync } from '$lib/sync';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  // Set base url as early as possible, or some components will accidentally use
  // the default url on a page reload.
  peerTubeApi.setConfig({
    baseUrl: PUBLIC_BF_PEERTUBE_URL,
  });

  /**
   * Client sessions are available for registered users and guests.
   *
   * There can be multiple client sessions per user, when they use multiple
   * devices, browsers, or browser profiles.
   *
   * To create a new guest session, the client has to make a request to the API server.
   * This could be changed in the future, to allow offline initialization.
   */

  // This state is read-only. Updates must go through setters. Otherwise, they
  // are not persisted.
  // Actually initialized in onMount
  /** @type {ClientSession} */
  const clientSession = $state({});
  const kvSync = new KvSync('bfkv_', updateMetaOnRemote, updateMetaInMemory);

  // Pwa auth works independent of the api server, generating a token to be used
  // for PeerTube only (for now).
  /** @type {PwaAuth} */
  const pwaAuth = $state({
    peerTubeToken: null,
    refreshPeerTubeToken,
  });

  function clearErrors() {
    loginError.title = '';
    loginError.description = '';
  }

  /** @type {import("$lib/peertube-openapi").User | {}} */
  const peerTubeUser = $state({});

  async function refreshPeerTubeToken() {
    if (loginError.title !== '') {
      // already has an error, probably doesn't have a session with the API server
      return;
    }

    const headers = {
      // Set a head even when it's an empty POST.
      // Triggers CORS pre-flight check to reduce CSRF risks.
      'Content-Type': 'application/json',
    };
    const body = '';

    peerTubeApi.setConfig({
      baseUrl: PUBLIC_BF_PEERTUBE_URL,
      auth: () => pwaAuth.peerTubeToken?.access_token,
    });

    try {
      const response = await authenticatedApiRequest(
        'POST',
        '/peertube/token',
        headers,
        body
      );

      if (response.okResponse) {
        pwaAuth.peerTubeToken = await response.okResponse.json();
      } else if (response.error !== API_ERROR.NeedLogin) {
        loginError.title = 'profile.login-failed';
        loginError.description = '';
        console.error(response.error);
        if (response.error === API_ERROR.BadGateway) {
          loginError.description = 'profile.login-failed-peertube-down';
        }
        return;
      }
    } catch (err) {
      if (err.error === API_ERROR.BadGateway) {
        loginError.description = 'profile.login-failed-peertube-down';
        return;
      }
      console.error('unexpected error calling authenticatedApiRequest', err);
    }
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
        meta: kvSync.load(),
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
            kvSync.setStringValue(
              'onboarding',
              ONBOARDING_STATE.FIRST_VISIT,
              new Date()
            );
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
          loginError.title = 'profile.guest-login-failed';
          loginError.description = 'profile.guest-login-failed-description';
          return {
            id: localStorage.clientSessionId,
            secret: localStorage.clientSessionSecret,
            meta: {
              onboarding: ONBOARDING_STATE.FIRST_VISIT,
            },
          };
        });
    }
  }

  async function syncKvWithServer() {
    const remoteData = await userCtx.authenticatedGet('/user/meta');
    if (remoteData && remoteData.ok) {
      const remoteMods = await remoteData.json();
      if (Array.isArray(remoteMods)) {
        await kvSync.sync(remoteMods);
      } else {
        console.error('Unexpected response from meta query', remoteMods);
      }
    } else {
      console.error('Meta query failed', remoteData);
    }
  }

  async function migrateFromFirstMetaStorage() {
    const oldMetaStr = localStorage.getItem('userMeta');
    if (oldMetaStr) {
      const oldMeta = parseOrNull(oldMetaStr);
      if (oldMeta) {
        // select an old date
        const date = new Date(2000, 0, 0, 0, 0);
        for (const [key, value] of Object.entries(oldMeta)) {
          // Only string values existed in the old version, so use that.
          // This also updates the remote.
          await kvSync.setStringValue(key, value, date);
        }
      }
    }
    localStorage.removeItem('userMeta');
  }

  function authHeader() {
    if ($user.openid) {
      // The user is not a guest, so shouldn't use client session login.
      // Instead, use cookie session.
      return {};
    }
    if (clientSession.id) {
      return {
        Authorization: `ClientSession ${clientSession.id}:${clientSession.secret}`,
      };
    }
    // This can happen when code calls this before clientSession is loaded.
    // Generally a programming error.
    console.warn(
      'Auth header could not be constructed,, missing client session'
    );
    return {};
  }

  /**
   * @param {string} path
   * @returns {Promise<Response | null | undefined>}
   */
  async function authenticatedGet(path) {
    const result = await authenticatedApiRequest('GET', path, {}, undefined);
    if (result?.okResponse) {
      return result.okResponse;
    }
    console.warn('get failed', result);
  }

  /**
   * @param {string} path
   * @param {object} body
   * @returns {Promise<Response | null | undefined>}
   */
  async function authenticatedPost(path, body) {
    const headers = {
      'Content-Type': 'application/json',
    };
    const result = await authenticatedApiRequest(
      'POST',
      path,
      headers,
      JSON.stringify(body)
    );
    if (result?.okResponse) {
      return result.okResponse;
    }
    console.warn('post failed', result);
  }

  /**
   * @param {string} method
   * @param {string} path
   * @param {object} headers
   * @param {string|FormData|undefined} body
   * @returns {Promise<import('$lib/stats').ApiResponse>}
   */
  async function authenticatedApiRequest(method, path, headers, body) {
    let auth = authHeader();
    const options = {
      method,
      headers: {
        ...headers,
        ...auth,
      },
      body,
    };

    const result = await apiRequest(path, options);

    if (result.error || result.errorBody) {
      console.error('result', result);
      switch (result.error) {
        case API_ERROR.NeedLogin: {
          // bubble up, RequiresLogin should handle this
          break;
        }
        case API_ERROR.UserNotFound: {
          // An older version may have lost the client session server-side. Use
          // local client state to recreate it with a new client session.
          // TODO: Remove this after a few updates, as this shouldn't be necessary
          // with the new code working.

          // try to get a new session
          const newSession = await requestNewGuestSession();

          // replace local session id and secret but keep other fields
          console.warn('replacing old session with a new');
          localStorage.clientSessionId = newSession.client_session_id;
          localStorage.clientSessionSecret = newSession.client_session_secret;
          clientSession.id = newSession.client_session_id;
          clientSession.secret = newSession.client_session_secret;

          // now update the server about our local state
          syncKvWithServer();

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
        case API_ERROR.ClientSessionLoginNotAllow: {
          // Must use keycloak login.
          // This shouldn't happen but if it does, we could trigger a refresh of user info.
          console.warn('Tried logging in as guest while being a full user.');
        }
        case API_ERROR.ClientSessionOfDifferentUser: {
          // TODO: Need user input to resolve.
          // Did the user mean to login to a different account?
          // What happens to locally stored data?
          // => Show $user.publicName and ask: log out and change user?
          // => About deleting:
          // - just user data and userMeta needs deleting, which should be
          //   synced to the sever. If a sync is pending, tell the user before
          //   they log out!
          // - steps / poses don't need to be deleted, can be kept locally
          // - localState can probably be kept, too
          break;
        }
        default:
          throw result;
      }
    }

    return result;
  }

  /**
   * @param {string} key
   * @param {string} value
   * @param {Date} lastModified
   * @param {number} version
   * @returns {Promise<void>}
   */
  async function updateMetaOnRemote(key, value, lastModified, version) {
    const headers = {
      'Content-Type': 'application/json',
    };
    const body = JSON.stringify({
      key_name: key,
      key_value: value,
      last_modified: lastModified,
      version,
    });

    const result = await authenticatedApiRequest(
      'POST',
      '/user/meta/update',
      headers,
      body
    );

    if (result.error || result.errorBody) {
      // TODO: handle login etc
      console.warn('meta update failed', result.errorBody);
    }

    return;
  }

  /**
   * @param {string} key
   * @param {string} value
   * @param {string} type
   * @param {Date} _lastModified
   * @param {number} _version
   */
  async function updateMetaInMemory(key, value, type, _lastModified, _version) {
    // for now, only handle strings here and avoid type conflicts
    // (the design might require some more iterations)
    if (type === 's:') {
      clientSession.meta[key] = value;
    }
  }

  /**
   * @param {string} key
   * @param {string} value
   * @returns {Promise<void>}
   */
  async function setUserMeta(key, value) {
    if (!key) {
      console.warn('Tried setting user meta with empty key');
      return;
    }

    // All updates go through sync object which first persists in local storage
    // and then updates in-memory and remote states.
    await kvSync.setStringValue(key, value, new Date());

    return;
  }

  /**
   * Read all user meta from the backend server.
   *
   * @returns {Promise<UserMetaResponse[] | undefined>}
   */
  async function getAllUserMeta() {
    const response = await authenticatedGet('/user/meta');

    if (response?.ok) {
      return response.json();
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
  // => Migrate this to user meta data and local data, as appropriate
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

  async function refreshUserId() {
    try {
      const response = await authenticatedGet('/user');
      const userInfo = await response?.json();
      if (!userInfo || userInfo.sub === undefined) {
        throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
      }

      // note: Sub may be null, this means the user is a guest.
      if (!userInfo.sub && $user.openid) {
        console.warn(
          'Client has a oidc subject but the server thinks this is a guest. Dropping previously known oidc subject.'
        );
      }
      $user.openid = userInfo.sub;

      // trigger subscribers?
      $user = $user;
    } catch (errResponse) {
      console.warn('Failed reading user info');
    }
  }

  async function refreshPeerTubeUser() {
    Object.assign(peerTubeUser, await fetchPeerTubeUser());
  }

  async function logout() {
    // TODO: handle local state without someone being logged in (without
    // creating a guest session -> be more explicit about guest sessions)
    pwaAuth.peerTubeToken = null;
    pwaAuth.refreshPeerTubeToken = null;

    const currentUrl = window.location.href;
    window.location.assign(
      PUBLIC_API_BASE +
        '/logout?redirect_back_to=' +
        encodeURIComponent(currentUrl)
    );
  }

  const loggedInToApi = $derived(!!pwaAuth.peerTubeToken);
  const isLoggedInToApi = () => loggedInToApi;
  let hasSkippedIntro = $state(false);
  let loginError = $state({ title: '', description: '' });

  /** @type {UserContextData} */
  const userCtx = {
    store: user,
    clientSession,
    pwaAuth,
    setUserMeta,
    submitWarmup,
    submitCourseLesson,
    submitStepTraining,
    addDanceToStats,
    isLoggedInToApi,
    refreshPeerTubeUser,
    logout,
    get peerTubeUser() {
      return (async () => {
        if (Object.keys(peerTubeUser).length === 0) {
          await refreshPeerTubeUser();
        }
        return peerTubeUser;
      })();
    },
    authenticatedGet,
    authenticatedPost,
    authenticatedApiRequest,
    skippedIntro: () => hasSkippedIntro,
    setSkippedIntro: (/** @type {boolean} */ yes) => (hasSkippedIntro = yes),
    loginError,
    clearErrors,
  };
  setContext('user', userCtx);

  // TODO: Anti-pattern of waiting for onMount to render.
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

    // migration from old to new way of storing user meta in local storage
    await migrateFromFirstMetaStorage();
    await syncKvWithServer();
  });
</script>

{#if has.mounted}
  {@render children?.()}
{/if}
