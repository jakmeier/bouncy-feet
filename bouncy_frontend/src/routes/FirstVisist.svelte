<script>
  import { getContext } from 'svelte';

  const localState = getContext('localState');

  function continueAsGuest() {
    localState.onboarding.firstVisit = false;
  }
</script>

<h3>TODO: First time visit</h3>

<button onclick={continueAsGuest}>kk</button>
