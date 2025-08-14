<script>
  import { t } from '$lib/i18n.js';

  
  /**
   * @typedef {Object} Props
   * @property {any} [title]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { title = undefined, children } = $props();
</script>

<div id="container">
  {#if title}
    <p>{$t(title)}</p>
  {/if}
  <div class="inner">
    {@render children?.()}
  </div>
</div>

<style>
  p {
    background-color: var(--theme-neutral-dark-gray);
    color: var(--theme-neutral-dark);
    margin: 0;
  }
  #container {
    min-width: 200px;
    text-align: center;
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-gray);
    width: 100%;
  }
  .inner {
    display: flex;
    flex-direction: column;
    min-height: 100px;
    padding: 20px;
    align-items: center;
    gap: 10px;
  }
</style>
