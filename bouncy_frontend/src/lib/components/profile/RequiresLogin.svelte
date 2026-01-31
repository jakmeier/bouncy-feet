<script>
  import { t } from '$lib/i18n';
  import { PUBLIC_API_BASE } from '$env/static/public';
  import { USER_AUH_STATE } from '$lib/enum_types';
  import { asset } from '$app/paths';

  /**
   * `RequiresLogin` is shown when the user needs to take an action to register,
   * log in, or refresh an existing session.
   *
   * This component does not check if a login is required, the parent dictates
   * this with props.
   *
   * Trying establishing auth automatically should have already concluded by the
   * time this is shown.
   */

  /**
   * @typedef {Object} Props
   * @property {string} [reason]
   * @property {string} [username]
   * @property {UserAuthState} authState
   * @property {boolean} guestAllowed
   * @property {boolean} prefersFullUserView
   * @property {()=>void} requestNewGuestSession
   */

  /** @type {Props} */
  let {
    reason,
    username,
    authState,
    guestAllowed,
    prefersFullUserView,
    requestNewGuestSession,
  } = $props();

  /** @type {string[]} */
  const displayedText = $derived.by(() => {
    const out = [];
    switch (authState) {
      case USER_AUH_STATE.Anonymous:
        out.push('profile.new-device-text');
        break;
      case USER_AUH_STATE.Guest:
        // If we get here, a full user is required.
        console.assert(
          !guestAllowed,
          'RequiresLogin is open with a user in guest mode, when guest mode is allowed'
        );
        out.push('profile.guest-mode-text');
        out.push('profile.requires-full-user-text');
        break;
      case USER_AUH_STATE.SignedUpUserExpiredAPISession:
        // Need an active API session to make API requests
        out.push('profile.login-expired-text');
        break;
      case USER_AUH_STATE.SignedUpUserExpiredPeerTubeSession:
        // TODO: why are we here exactly?
        break;
      case USER_AUH_STATE.SignedUpUser:
        console.assert(
          false,
          'RequiresLogin is open with a user fully logged in.'
        );
        break;
      default:
        console.assert(false, 'Unknown auth state');
    }

    return out;
  });

  function login() {
    // redirect to backend login
    const currentUrl = window.location.href;
    window.location.assign(
      PUBLIC_API_BASE +
        '/login?redirect_back_to=' +
        encodeURIComponent(currentUrl)
    );
  }

  function register() {
    // redirect to backend register -> login
    const currentUrl = window.location.href;
    window.location.assign(
      PUBLIC_API_BASE +
        '/register?redirect_back_to=' +
        encodeURIComponent(currentUrl)
    );
  }

  function goBack() {
    history.back();
  }

  function createGuest() {
    if (authState != USER_AUH_STATE.Anonymous) {
      const p0 = $t('profile.previous-login-name');
      const p1 = $t('profile.switch-to-guest-warning');
      const warning = `${p0} ${username}\n${p1}`;
      if (!confirm(warning)) {
        return;
      }
    }
    requestNewGuestSession();
  }
</script>

<img
  class="header-logo"
  src={asset('/icons/icon_tight_on_transparent.png')}
  alt="Bouncy Feet Logo"
/>

<img
  class="footer-logo"
  src={asset('/icons/bouncyfeet_white.svg')}
  alt="Bouncy Feet Text Logo"
/>

{#if username}
  Hi {username}!
{/if}

<button class="wide login" onclick={login}>
  {$t('profile.button-login')}
</button>

{#each displayedText as text}
  <div>
    {$t(text)}
  </div>
{/each}

{#if guestAllowed}
  <button class="wide light" onclick={createGuest}>
    {$t('profile.button-create-guest')}
  </button>
{/if}

<button class="wide light" onclick={register}>
  {$t('profile.button-register')}
</button>
<button class="wide light" onclick={goBack}>
  {$t('profile.button-cancel')}
</button>

<div>
  {#if reason}
    {reason}
  {/if}
</div>

<style>
  .header-logo {
    margin-bottom: 2rem;
    max-width: 200px;
  }

  .login {
    margin-bottom: 2rem;
  }

  .footer-logo {
    position: absolute;
    bottom: 1rem;
    max-width: 200px;
  }
</style>
