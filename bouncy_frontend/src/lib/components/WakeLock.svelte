<script>
  /**
   * Using this component on a site ensures that the screen does not go to sleep without user interaction.
   */

  import { onDestroy, onMount } from 'svelte';

  /** @type {WakeLockSentinel | null} */
  let wakeLock = null;
  let isActive = false;

  async function requestWakeLock() {
    try {
      if ('wakeLock' in navigator) {
        wakeLock = await navigator.wakeLock.request('screen');
        isActive = true;

        // Listening for wake lock release, as the system might decide to
        // release it for power saving reasons or whatever.
        wakeLock.addEventListener('release', () => {
          isActive = false;
        });
      } else {
        console.warn('Wake Lock API is not supported in this browser.');
      }
    } catch (err) {
      console.error('Wake Lock failed:', err);
    }
  }

  function releaseWakeLock() {
    if (wakeLock) {
      wakeLock.release();
      wakeLock = null;
      isActive = false;
    }
  }

  onMount(() => {
    requestWakeLock();
  });

  onDestroy(() => {
    releaseWakeLock();
  });
</script>
