<script>
  import { t } from '$lib/i18n';
  import Popup from './Popup.svelte';

  export let title;
  export let text;

  /** @type {import('svelte/store').Writable<boolean>} */
  let isOpen;

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
    on:click={toggleOpen}
    on:keydown={toggleOpen}
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
  <slot />
</Popup>

<style>
  .info span {
    font-size: 32px;
    cursor: pointer;
  }
</style>
