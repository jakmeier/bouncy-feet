<script>
  /**
   * UserContext provides access to user info.
   *
   * It is set once at the top most layout and shared throughout all pages. Use
   * `LoginRequiredContent` guards to access it while ensuring a level of
   * registration with an active session.
   */
  import { setContext } from 'svelte';
  import { client as peerTubeApi } from '$lib/peertube-openapi/client.gen';
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { showExperimentalFeatures } from './FeatureSelection';
  import { generateRandomUsername } from '$lib/username';
  import { browser } from '$app/environment';
  import { USER_AUH_STATE } from '$lib/enum_types';
  import { ApiUser } from './ApiUser.svelte';
  import { ONBOARDING_STATE } from '$lib/onboarding';

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

  // TODO: The user object seems outdated, no longer fitting the rest of the
  // architecture. It should also change to only generate a new name when a new
  // guest session is created. Also check how these stats are actually used now.
  /** @type {UserData} */
  const user = $state(
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

  $effect(() => {
    localStorage.user = JSON.stringify(user);
    showExperimentalFeatures(user.experimentalFeatures);
  });

  let apiUserLock = false;
  /** @returns {Promise<ApiUser|undefined>} */
  async function maybeLoadApiUser() {
    if (!userCtx.apiUser) {
      if (!apiUserLock) {
        apiUserLock = true;
        const tmpApiUser = await ApiUser.initApiUser(false, userCtx);

        // make the first use of the API user to see if it has a valid session
        // (returning users can restore a client session but won't be logged in)
        if (tmpApiUser) {
          let err = await tmpApiUser.syncKvWithServer();
          if (err) {
            apiUserLock = false;
            return undefined;
          }
        }

        userCtx.apiUser = tmpApiUser;
        apiUserLock = false;

        if (userCtx.apiUser) {
          // migration from old to new way of storing user meta in local storage
          await userCtx.apiUser.clientSession.migrateFromFirstMetaStorage(
            userCtx.apiUser.kvSync
          );
        }
      } else {
        while (!userCtx.apiUser) {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      }
    }
    return userCtx.apiUser;
  }

  async function createGuestUser() {
    console.assert(
      !userCtx.apiUser,
      'creating a guest while an account already exists'
    );

    while (apiUserLock) {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    apiUserLock = true;
    const apiUser = await ApiUser.initApiUser(true, userCtx);
    console.assert(!!apiUser, 'creating a new guest API user failed');
    userCtx.apiUser = apiUser;
    apiUserLock = false;

    // note: apiUser should have an active session now, as it was just created.
    if (apiUser) {
      apiUser.kvSync.setStringValue(
        'onboarding',
        ONBOARDING_STATE.FIRST_VISIT,
        new Date()
      );
      apiUser.kvSync.setStringValue(
        'publicName',
        apiUser.userCtx.user.publicName,
        new Date()
      );
    }
  }

  /** @type {UserContextData} */
  const userCtx = $state({
    user,
    apiUser: undefined,
    fullUser: undefined,
    get authState() {
      return userAuthState;
    },
    logout: () => {
      userCtx.fullUser?.logout();
      userCtx.apiUser = undefined;
      userCtx.fullUser = undefined;
    },
    maybeLoadApiUser,
    createGuestUser,
  });
  setContext('user', userCtx);

  /**
   * @returns {UserAuthState}
   */
  const userAuthState = $derived.by(() => {
    const apiUserLoaded = !!userCtx.apiUser;
    const hasOpenId = !!userCtx.user.openid;
    const hasPeerTubeToken =
      !!userCtx?.fullUser?.pwaAuth.peerTubeToken?.access_token;

    switch (true) {
      case !apiUserLoaded && !hasOpenId:
        return USER_AUH_STATE.Anonymous;
      case apiUserLoaded && !hasOpenId:
        return USER_AUH_STATE.Guest;
      case !apiUserLoaded && hasOpenId:
        return USER_AUH_STATE.SignedUpUserExpiredAPISession;
      case apiUserLoaded && hasOpenId && !hasPeerTubeToken:
        return USER_AUH_STATE.SignedUpUserExpiredPeerTubeSession;
      case apiUserLoaded && hasOpenId && hasPeerTubeToken:
        return USER_AUH_STATE.SignedUpUser;
      default:
        console.warn('Unhandled user state', [
          apiUserLoaded,
          hasOpenId,
          hasPeerTubeToken,
        ]);
        return USER_AUH_STATE.Anonymous;
    }
  });
</script>

{@render children?.()}
