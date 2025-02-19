<script>
  import { getContext } from 'svelte';

  const { localState } = getContext('user');

  function continueAsGuest() {
    $localState.firstVisit = false;
    // trigger subscribers
    $localState = $localState;
  }
</script>

<h3>TODO: First time visit</h3>

<button on:click={continueAsGuest}>kk</button>
