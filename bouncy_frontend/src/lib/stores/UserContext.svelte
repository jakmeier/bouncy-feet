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

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  // let loginError = $state({ title: '', description: '' });

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

  /**
   * @returns {UserAuthState}
   */
  const userAuthState = () => {
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
  };

  /** @type {UserContextData} */
  const userCtx = {
    user,
    apiUser: undefined,
    fullUser: undefined,
    get authState() {
      return userAuthState();
    },
  };
  setContext('user', userCtx);
</script>

{@render children?.()}
