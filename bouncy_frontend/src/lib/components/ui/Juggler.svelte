<script>
  import JuggleElement from './JuggleElement.svelte';

  /**
   * @typedef {Object} Props
   * @property {(index: number)=>void} [onIndexChanged]
   * @property {any[]} items
   * @property {import('svelte').Snippet<[any]>} element
   * @property {string} [height]
   * @property {string} [width]
   * @property {string} [buttonHeight]
   */

  /** @type {Props} */
  let {
    onIndexChanged = () => {},
    items,
    element,
    width = '100%',
    height = '100%',
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
</script>

<div
  class="container"
  style="--component-width: {width}; --component-height: {height}; --button-height: {buttonHeight};"
>
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
    width: var(--component-width);
    height: var(--component-height);
  }

  .container button {
    position: absolute;
    z-index: 1;
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

  .videos {
    overflow: hidden;
  }
</style>
