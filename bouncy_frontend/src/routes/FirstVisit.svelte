<script>
  import { getContext } from 'svelte';

  /** @type {UserContextData}*/
  const { setClientSessionMeta } = getContext('user');

  function continueAsGuest() {
    // TODO: actually go to first warmup
    // setClientSessionMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_WARMUP);
  }
</script>

<h3>TODO: First time visit</h3>

<button onclick={continueAsGuest}>kk</button>
