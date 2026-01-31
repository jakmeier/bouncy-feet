<script>
  /**
   * Component to guard inner content to only give access when the necessary user
   * info is available.
   *
   * ## User login levels
   *
   * 1. Anonymous: Users start anonymous. In this state, they can browse public
   *    information.
   * 2. Guest: To record activities, a client session is required, which in turn
   *    requires at least a guest account.
   * 3. Full account: A full account is required for any social features, such as
   *    uploading videos or joining a club.
   */

  import { getUserContext } from '$lib/stores/context';
  import { onMount } from 'svelte';
  import { ApiUser } from '$lib/stores/ApiUser.svelte';
  import { initFullUser } from '$lib/stores/FullUser.svelte';
  import { USER_AUH_STATE } from '$lib/enum_types';
  import { fade } from 'svelte/transition';
  import { scaleY } from '$lib/sveltex/xtransition';
  import { t } from '$lib/i18n';
  import LoginError from './LoginError.svelte';
  import RequiresLogin from './RequiresLogin.svelte';
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { apiRequest } from '$lib/stats';

  /**
   * @typedef {Object} Props
   * @property {string} [reason]
   * @property {import('svelte').Snippet<[{ apiUser: ApiUser, user: UserData }]>} [guest]
   * @property {import('svelte').Snippet<[{ apiUser: ApiUser, maybeFullUser: FullUser | undefined, user: UserData}]>} [maybeFullUser]
   * @property {import('svelte').Snippet<[{ apiUser: ApiUser, fullUser: FullUser, user: UserData}]>} [children]
   */

  /** @type {Props} */
  let {
    reason,
    guest: guestContent,
    maybeFullUser: maybeFullUserContent,
    children: fullUserContent,
  } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  /**
   * @type {BfError} loginError -- only set if login is impossible, otherwise just trigger a login
   */
  let loginError = $state({
    title: '',
    description: '',
  });

  function clearErrors() {
    loginError.title = '';
    loginError.description = '';
  }

  /**
   * @returns {Promise<import('$lib/stats').ApiError | undefined>}
   */
  async function refreshUserId() {
    let apiResponse;
    if (!userCtx.apiUser) {
      apiResponse = await apiRequest('/user');
    } else {
      apiResponse = await userCtx.apiUser.authenticatedGet('/user');
    }

    if (apiResponse.error) {
      return apiResponse.error;
    }

    let response = apiResponse.okResponse;
    try {
      const userInfo = await response?.json();
      if (!userInfo || userInfo.sub === undefined) {
        throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
      }

      // note: Sub may be null, this means the user is a guest.
      if (!userInfo.sub && userCtx.user.openid) {
        console.warn(
          'Client has a oidc subject but the server thinks this is a guest. Dropping previously known oidc subject.'
        );
      }
      userCtx.user.openid = userInfo.sub;
    } catch (errResponse) {
      console.warn('Failed reading user info');
    }
  }

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

  let fullUserLock = false;
  /**
   * @param {ApiUser} apiUser
   * @returns {Promise<FullUser>}
   */
  async function ensureFullUser(apiUser) {
    if (!userCtx.fullUser) {
      if (!fullUserLock) {
        fullUserLock = true;
        userCtx.fullUser = await initFullUser(apiUser);
      } else {
        while (!userCtx.fullUser) {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      }
    }
    return userCtx.fullUser;
  }

  /**
   * @returns {Promise<string>}
   */
  async function peerTubeToken() {
    // TODO: Do I need this check?
    // if (loginError.title !== '') {
    //     // already has an error, probably doesn't have a session with the API server
    //     return;
    // }

    let result = await userCtx.fullUser?.peerTubeToken();

    if (result && 'title' in result) {
      loginError.title = result.title;
      loginError.description = result.description;
    }

    if (result && 'accessToken' in result && result.accessToken) {
      return result.accessToken;
    }

    // throw new Error("Error");
  }

  // While trying to establish the required auth state automatically, a loading
  // animation is shown. Once that's done, successful or not, `loading` is set to false.
  // If an error occurs, a `loginError` value is set.
  let loading = $state(true);

  const guestAllowed = $derived(!!guestContent || !!maybeFullUserContent);
  const prefersFullUserView = $derived(
    !!maybeFullUserContent || !!fullUserContent
  );

  const hasGuestAuth = $derived(!!userCtx.apiUser);
  const hasFullAuth = $derived(
    userCtx.authState === USER_AUH_STATE.SignedUpUser
  );

  onMount(async () => {
    if (!userCtx.user.openid) {
      let err = await refreshUserId();

      if (err) {
        loading = false;
        return;
      }
    }

    const apiUser = await maybeLoadApiUser();

    if (!apiUser) {
      loading = false;
      return;
    }

    // TODO: Maybe check first if a sync is needed?
    // await apiUser.syncKvWithServer();
    userCtx.user.publicName = apiUser.meta['publicName'];

    if (fullUserContent || maybeFullUserContent) {
      const fullUser = await ensureFullUser(apiUser);
      clearErrors();
      const _token = await fullUser.peerTubeToken();
      // // TODO: prevent this from firing too often
      // if (!fullUser.isLoggedInToApi()) {
      //   try {
      //     const _token = await fullUser.peerTubeToken();
      //   } catch (e) {
      //     console.debug('failed to refresh PeerTube token', e);
      //   }
      // }
    }

    loading = false;
  });
</script>

{#if loading}
  {$t('profile.waiting-for-login-info')}
{:else if fullUserContent && hasFullAuth}
  {@render fullUserContent({
    apiUser: userCtx.apiUser,
    fullUser: userCtx.fullUser,
    user: userCtx.user,
  })}
{:else if maybeFullUserContent && hasFullAuth}
  {@render maybeFullUserContent({
    apiUser: userCtx.apiUser,
    maybeFullUser: userCtx.fullUser,
    user: userCtx.user,
  })}
  <!-- {:else if hasGuestAuth && prefersFullUserView} -->
  <!-- TODO: tell user that they could log in to see more -->
{:else if maybeFullUserContent && hasGuestAuth}
  {@render maybeFullUserContent({
    apiUser: userCtx.apiUser,
    user: userCtx.user,
    maybeFullUser: undefined,
  })}
{:else if guestContent && hasGuestAuth}
  {@render guestContent({
    apiUser: userCtx.apiUser,
    user: userCtx.user,
  })}
{:else}
  <div class="overlay" transition:fade={{ delay: 400, duration: 200 }}>
    <div class="wrapper" transition:scaleY={{ delay: 800 }}>
      {#if loginError.title !== ''}
        <LoginError {loginError}></LoginError>
      {:else}
        <RequiresLogin
          authState={userCtx.authState}
          {guestAllowed}
          {prefersFullUserView}
          requestNewGuestSession={createGuestUser}
        />
      {/if}
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 19999;
  }

  .wrapper {
    display: flex;
    background-color: var(--background-color, var(--theme-neutral-dark));
    color: var(--color, var(--theme-neutral-gray));
    flex-direction: column;
    min-height: 100px;
    padding: 20px;
    align-items: center;
    text-align: center;
    gap: 10px;
  }
</style>
