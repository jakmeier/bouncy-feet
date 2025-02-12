<script>
  import { t } from '$lib/i18n.js';

  /** @type{String|undefined} */
  export let title = undefined;
</script>

<div id="container">
  {#if title}
    <p>{$t(title)}</p>
  {/if}
  <div class="inner">
    <slot />
  </div>
</div>

<style>
  p {
    background-color: var(--theme-neutral-black);
    color: var(--theme-neutral-gray);
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
