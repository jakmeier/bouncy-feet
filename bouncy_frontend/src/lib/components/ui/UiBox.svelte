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
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    margin: 0;
  }
  #container {
    min-width: 200px;
    text-align: center;
    background-color: var(--theme-neutral-light);
    width: 100%;
  }
  .inner {
    display: flex;
    flex-direction: column;
    min-height: 100px;
    padding: 10px;
    align-items: center;
    gap: 10px;
  }
</style>
