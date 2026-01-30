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

  import { t } from '$lib/i18n';
  import { getUserContext } from '$lib/stores/context';
  import RequiresLogin from './RequiresLogin.svelte';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { scaleY } from '$lib/sveltex/xtransition';
  import { asset } from '$app/paths';
  import { goto } from '$app/navigation';
  import { ApiUser } from '$lib/stores/ApiUser.svelte';
  import { initFullUser } from '$lib/stores/FullUser.svelte';

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

  async function refreshUserId() {
    try {
      const response = await userCtx.apiUser.authenticatedGet('/user');
      const userInfo = await response?.json();
      if (!userInfo || userInfo.sub === undefined) {
        throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
      }

      // note: Sub may be null, this means the user is a guest.
      if (!userInfo.sub && userCtx.apiUser.user.openid) {
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
  /**
   * @returns {Promise<ApiUser>}
   */
  async function ensureApiUser() {
    if (!userCtx.apiUser) {
      if (!apiUserLock) {
        apiUserLock = true;
        userCtx.apiUser = await ApiUser.initApiUser(true, userCtx);
        await userCtx.apiUser?.syncKvWithServer();
      } else {
        while (!userCtx.apiUser) {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      }
    }
    return userCtx.apiUser;
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

  let mounted = $state(false);
  onMount(async () => {
    const apiUser = await ensureApiUser();
    if (!userCtx.user.openid) {
      await refreshUserId();
    }

    if (fullUserContent) {
      const fullUser = await ensureFullUser(apiUser);
      // TODO: Is this necessary?
      // clearErrors();
      // // TODO: prevent this from firing too often
      // if (!fullUser.isLoggedInToApi()) {
      //   try {
      //     await fullUser.refreshPeerTubeToken();
      //   } catch (e) {
      //     console.debug('failed to refresh PeerTube token', e);
      //   }
      // }
    }

    mounted = true;
  });
</script>

{#if mounted && fullUserContent && userCtx.fullUser && userCtx.apiUser && userCtx.fullUser.isLoggedInToApi()}
  {@render fullUserContent?.({
    apiUser: userCtx.apiUser,
    fullUser: userCtx.fullUser,
    user: userCtx.user,
  })}
{:else if mounted && maybeFullUserContent && userCtx.apiUser}
  {@render maybeFullUserContent?.({
    apiUser: userCtx.apiUser,
    maybeFullUser: userCtx.fullUser,
    user: userCtx.user,
  })}
{:else if mounted && guestContent && userCtx.apiUser}
  {@render guestContent?.({
    apiUser: userCtx.apiUser,
    user: userCtx.user,
  })}
{:else if mounted}
  <div class="overlay" transition:fade={{ delay: 400, duration: 200 }}>
    <div class="wrapper" transition:scaleY={{ delay: 800 }}>
      {#if loginError.title !== ''}
        <h2>{$t(loginError.title)}</h2>
        <div class="error-symbol">
          <img src={asset('/img/symbols/bf_error.svg')} alt="bf_error" />
        </div>
        <div class="block">
          {#if loginError.description}
            <div>{$t(loginError.description)}</div>
          {/if}
          <div>{$t('common.sorry-error')}</div>
        </div>
        <button onclick={() => history.back()}>
          {$t('common.go-to-prev-page-button')}
        </button>
        <button onclick={() => goto('/')}>
          {$t('common.go-home-button')}
        </button>
        <div class="block">
          <div>{$t('common.report-error')}</div>
        </div>
      {:else}
        <RequiresLogin
          {reason}
          username={userCtx.user.publicName}
          openid={userCtx.user.openid}
        />
      {/if}
    </div>
  </div>
{:else}
  {$t('profile.waiting-for-login-info')}
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

  .error-symbol img {
    max-width: 6rem;
  }

  .block {
    padding: 1rem 0;
  }
</style>
