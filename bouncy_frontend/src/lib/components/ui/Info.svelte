<script>
  import { t } from '$lib/i18n';
  import Popup from './Popup.svelte';

  let { title, text, children } = $props();

  /** @type {import('svelte/store').Writable<boolean>} */
  let isOpen = $state();

  /** @param {KeyboardEvent | MouseEvent} event */
  function toggleOpen(event) {
    if (
      event.type === 'click' ||
      (event.type === 'keydown' &&
        event instanceof KeyboardEvent &&
        (event.key === 'Enter' || event.key === ' '))
    ) {
      $isOpen = !$isOpen;
    }
  }
</script>

<div class="info">
  <span
    class="material-symbols-outlined"
    translate="no"
    onclick={toggleOpen}
    onkeydown={toggleOpen}
    role="button"
    tabindex="0"
    aria-expanded={$isOpen}
  >
    info
  </span>
</div>

<Popup {title} bind:isOpen showOkButton>
  <div>
    {$t(text)}
  </div>
  {@render children?.()}
</Popup>

<style>
  .info span {
    font-size: var(--font-large);
    cursor: pointer;
  }
</style>
