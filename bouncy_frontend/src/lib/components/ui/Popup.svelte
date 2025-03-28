<!-- Popup.svelte -->
<script>
  import { writable } from 'svelte/store';
  import UiBox from '$lib/components/ui/UiBox.svelte';

  /**
   * @typedef {Object} Props
   * @property {import("svelte/store").Writable<boolean>} [isOpen]
   * @property {any} [title]
   * @property {boolean} [showOkButton]
   * @property {function} [onClose]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    isOpen = $bindable(writable(false)),
    title = undefined,
    showOkButton = false,
    onClose,
    children,
  } = $props();

  function handleClose() {
    isOpen.set(false);
    if(onClose) {
      onClose();
    }
  }

  // Optionally close the popup when clicking outside of it
  function handleClickOutside(event) {
    if (!event.target.closest('.popup-content')) {
      handleClose();
    }
  }
</script>

{#if $isOpen}
  <div class="overlay" onclick={handleClickOutside}>
    <div class="popup-content">
      <UiBox {title}>
        {@render children?.()}
        {#if showOkButton}
          <button onclick={handleClose}>OK</button>
        {/if}
      </UiBox>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }

  .popup-content {
    padding: 0;
    margin: 10px;
    border-radius: 5px;
    overflow: auto;
    z-index: 10000;
  }

  button {
    width: 50px;
    height: 50px;
    padding: auto;
  }
</style>
