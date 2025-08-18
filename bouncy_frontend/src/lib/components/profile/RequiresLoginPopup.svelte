<script>
  import { triggerLogin, triggerRegister } from '$lib/keycloak';
  import { t } from '$lib/i18n';
  import PopupWithRunes from '../ui/PopupWithRunes.svelte';
  import { getUserContext } from '$lib/context';

  let { reason } = $props();

  const userContext = getUserContext();

  /** @type {boolean} */
  let notLoggedIn = $derived(!userContext.loggedInToKeycloak());

  function login() {
    triggerLogin();
  }

  function register() {
    triggerRegister();
  }

  function goBack() {
    history.back();
  }
</script>

<PopupWithRunes
  bind:isOpen={notLoggedIn}
  title={'profile.requires-login-title'}
  onClose={goBack}
>
  <div>
    {reason}
  </div>
  <div>
    {$t('profile.requires-login-text')}
  </div>
  <button class="wide" onclick={login}>
    {$t('profile.button-login')}
  </button>
  <button class="wide" onclick={register}>
    {$t('profile.button-register')}
  </button>
  <button class="wide" onclick={goBack}>
    {$t('profile.button-cancel')}
  </button>
</PopupWithRunes>
