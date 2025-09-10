<script>
  import PopupWithRunes from '../ui/PopupWithRunes.svelte';
  import { getUserContext } from '$lib/context';
  import RequiresLogin from './RequiresLogin.svelte';

  let { reason } = $props();

  const userContext = getUserContext();
  const user = userContext.store;

  /** @type {boolean} */
  let notLoggedIn = $state(userContext.isLoggedInToApi());

  function goBack() {
    history.back();
  }
</script>

<PopupWithRunes
  bind:isOpen={notLoggedIn}
  title={'profile.requires-login-title'}
  onClose={goBack}
>
  <RequiresLogin {reason} />
</PopupWithRunes>
