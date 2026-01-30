<script>
  import { t } from '$lib/i18n';
  import { PUBLIC_API_BASE } from '$env/static/public';

  /**
   * @typedef {Object} Props
   * @property {string} [reason]
   * @property {string} [username]
   * @property {string} [openid]
   */

  /** @type {Props} */
  let { reason, username, openid } = $props();

  const guestMode = $derived(!openid);

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
</script>

{#if guestMode}
  <div>
    {$t('profile.guest-mode-text')}
  </div>
  <div>
    {$t('profile.requires-login-text')}
  </div>
{:else}
  Hi {username}!
  <div>
    {$t('profile.login-expired-text')}
  </div>
{/if}

<button class="wide" onclick={login}>
  {$t('profile.button-login')}
</button>
<button class="wide" onclick={register}>
  {$t('profile.button-register')}
</button>
<button class="wide" onclick={goBack}>
  {$t('profile.button-cancel')}
</button>

<div>
  {#if reason}
    {reason}
  {/if}
</div>
