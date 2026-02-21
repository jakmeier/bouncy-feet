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
  import { ApiUser } from '$lib/stores/ApiUser.svelte';
  import { USER_AUH_STATE } from '$lib/enum_types';
  import { fade } from 'svelte/transition';
  import { scaleY } from '$lib/sveltex/xtransition';
  import { t } from '$lib/i18n';
  import LoginError from './LoginError.svelte';
  import RequiresLogin from './RequiresLogin.svelte';
  import UserLoader from './UserLoader.svelte';

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

  /** @type {ApiUser|undefined} apiUser */
  let apiUser = $state();
  /** @type {FullUser|undefined} apiUser */
  let fullUser = $state();
  let apiUserLoader = $state();

  // While trying to establish the required auth state automatically, a loading
  // animation is shown. Once that's done, successful or not, `loading` is set to false.
  // If an error occurs, a `loginError` value is set.
  let loading = $state(true);

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

  const guestAllowed = $derived(!!guestContent || !!maybeFullUserContent);
  const prefersFullUserView = $derived(
    !!maybeFullUserContent || !!fullUserContent
  );

  const hasGuestAuth = $derived(!!userCtx.apiUser);
  const hasFullAuth = $derived(
    userCtx.authState === USER_AUH_STATE.SignedUpUser
  );

  /** @param {BfError} error */
  function setError(error) {
    loginError = error;
  }
</script>

<UserLoader
  bind:apiUser
  bind:fullUser
  bind:loading
  loadApiUser
  loadFullUser={!!(fullUserContent || maybeFullUserContent)}
  {setError}
/>

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
          requestNewGuestSession={apiUserLoader.createGuestUser}
          {reason}
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
