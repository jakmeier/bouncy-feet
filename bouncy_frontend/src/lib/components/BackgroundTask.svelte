<script>
  import { onDestroy, onMount } from 'svelte';

  /** @type {undefined | (() => void)} */
  export let onFrame = undefined;

  export function stop() {
    stopped = true;
  }

  let stopped = false;

  function frameLoop() {
    if (stopped || !onFrame) return;
    onFrame();
    requestAnimationFrame(frameLoop);
  }

  onMount(async () => {
    if (onFrame) {
      frameLoop();
    }
  });

  onDestroy(() => {
    stopped = true;
  });
</script>
