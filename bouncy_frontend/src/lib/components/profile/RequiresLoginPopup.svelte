<script>
  import { triggerLogin, triggerRegister } from '$lib/keycloak';
  import { t } from '$lib/i18n';
  import PopupWithRunes from '../ui/PopupWithRunes.svelte';
  import { getUserContext } from '$lib/context';
  import { PUBLIC_API_BASE } from '$env/static/public';

  let { reason } = $props();

  const userContext = getUserContext();

  /** @type {boolean} */
  let notLoggedIn = $derived(!userContext.loggedInToKeycloak());

  const opendId = $derived(userContext.pwaAuth.userProfile?.id);
  const user = userContext.store;
  // const hasKeycloakAccountLinked = ???;

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
    // TODO: replace with backend register
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
