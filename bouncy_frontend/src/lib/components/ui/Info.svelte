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

  function close() {
    $isOpen = false;
  }
</script>

<div class="info">
  <span
    class="material-symbols-outlined"
    on:click={toggleOpen}
    on:keydown={toggleOpen}
    role="button"
    tabindex="0"
    aria-expanded={$isOpen}
  >
    info
  </span>
</div>

<Popup {title} bind:isOpen>
  <div>
    {$t(text)}
  </div>
  <slot />
  <button on:click={close}>OK</button>
</Popup>

<style>
  .info span {
    font-size: 32px;
    cursor: pointer;
  }

  button {
    width: 50px;
    height: 50px;
    padding: auto;
    background-color: var(--theme-accent-light);
  }
</style>
