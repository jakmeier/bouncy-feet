<script>
  import JuggleElement from './JuggleElement.svelte';
  import { useSwipe } from 'svelte-gestures';

  /**
   * @typedef {Object} Props
   * @property {(index: number)=>void} [onIndexChanged]
   * @property {any[]} items
   * @property {import('svelte').Snippet<[any]>} element
   * @property {string} [buttonHeight]
   */

  /** @type {Props} */
  let {
    onIndexChanged = () => {},
    items,
    element,
    buttonHeight = '50%',
  } = $props();
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

  /**@param {import('svelte-gestures').SwipeCustomEvent} event */
  function handler(event) {
    if (event.detail.direction === 'right') {
      prev();
    }
    if (event.detail.direction === 'left') {
      next();
    }
  }
</script>

<div class="container" style="--button-height: {buttonHeight};">
  {#if items.length > 1}
    <button onclick={prev}>&lt;</button>
  {/if}
  <div
    class="elements"
    {...useSwipe(handler, () => ({
      timeframe: 300,
      minSwipeDistance: 50,
      touchAction: 'none',
    }))}
  >
    {#each items as item, index}
      <JuggleElement position={pos(index)} {index}>
        {@render element({ item, index })}
      </JuggleElement>
    {/each}
  </div>
  {#if items.length > 1}
    <button onclick={next}>&gt;</button>
  {/if}
</div>

<style>
  .container {
    position: relative;
    height: 100%;
    width: 100%;
  }

  .container button {
    position: absolute;
    z-index: 10001;
    top: var(--button-height);
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

  .elements {
    overflow: hidden;
    display: flex;
    position: relative;
  }
</style>
