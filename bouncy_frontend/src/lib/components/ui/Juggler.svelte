<script>
  import JuggleElement from './JuggleElement.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {(index: number)=>void} [onIndexChanged]
   * @property {any[]} items
   * @property {import('svelte').Snippet<[any]>} element
   */

  /** @type {Props} */
  let { onIndexChanged = () => {}, items, element } = $props();
  let currentIndex = $state(0);

  export function prev() {
    currentIndex = (currentIndex + items.length - 1) % items.length;
    onIndexChanged(currentIndex);
  }

  export function next() {
    currentIndex = (currentIndex + 1) % items.length;
    onIndexChanged(currentIndex);
  }

  /**
   * @param {number} index
   * @returns {"left"|"right"|"center"}
   */
  function pos(index) {
    if (index < currentIndex) {
      return 'left';
    }
    if (index > currentIndex) {
      return 'right';
    }
    return 'center';
  }

  /**
   * Loading all at once may hit rate limits, so delay loading
   * @param {number} index
   * @return {number} ms
   */
  function delayMs(index) {
    const delta = Math.abs(index - currentIndex);
    if (delta < 3) {
      // Load the first few fairly quickly
      return delta * 500;
    } else {
      // delay the rest for much longer
      return (delta - 2) * 5000;
    }
  }
</script>

<div class="container">
  <button onclick={prev}>&lt;</button>
  <div class="videos">
    {#each items as item, reverseIndex}
      {@const index = items.length - 1 - reverseIndex}
      <JuggleElement position={pos(index)}>
        {@render element({ item, index })}
      </JuggleElement>
    {/each}
  </div>
  <button onclick={next}>&gt;</button>
</div>

<style>
  .container {
    position: relative;
    width: 100%;
    height: min(70vh, 100%);
  }

  .container button {
    position: absolute;
    z-index: 1;
    top: 50%;
    width: 3rem;
    height: 3rem;
    padding: 0;
    margin: 0;
    border-radius: 50%;
    min-width: initial;
    max-width: initial;
  }

  .container button:first-child {
    left: -1.5rem;
  }
  .container button:last-child {
    right: -1.5rem;
  }

  .videos {
    overflow: hidden;
  }
</style>
