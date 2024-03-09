<script>
  import { t } from '$lib/i18n.js';

  /** @type{String|undefined} */
  export let title;
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
  }
  #container {
    text-align: center;
    background-color: var(--theme-neutral-light);
    width: 100%;
  }
</style>
