<script>
  import { onDestroy, onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {undefined | (() => void)} [onFrame]
   */

  /** @type {Props} */
  let { onFrame = undefined } = $props();

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
