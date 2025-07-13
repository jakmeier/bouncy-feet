<script lang="ts">
  import JuggleElement from './JuggleElement.svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} ids
   * @property {import('svelte').Snippet<[any]>} [items]
   */

  /** @type {Props} */
  let { ids, items } = $props();
  let currentIndex = $state(0);
  $inspect(currentIndex);

  function next() {
    currentIndex = (currentIndex + 1) % ids.length;
  }

  function pos(index) {
    if (index < currentIndex) {
      return -150;
    }
    if (index > currentIndex) {
      return 150;
    }
    return 0;
  }
</script>

<div class="container">
  {#each ids as id, index}
    <JuggleElement x={pos(index)}>
      {@render items(id)}
    </JuggleElement>
  {/each}
</div>

<button onclick={next}>Next</button>

<style>
  .container {
    position: relative;
    width: 100vw;
    height: 60vh;
    overflow: hidden;
  }
</style>
