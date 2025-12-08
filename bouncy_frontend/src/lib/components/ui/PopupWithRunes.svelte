<script>
  import UiBox from '$lib/components/ui/UiBox.svelte';

  /**
   * @typedef {Object} Props
   * @property {boolean} [isOpen]
   * @property {any} [title]
   * @property {boolean} [showOkButton]
   * @property {function} [onClose]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    isOpen = $bindable(false),
    title = undefined,
    showOkButton = false,
    onClose,
    children,
  } = $props();

  let overlay = $state();

  // Set focus for a11y and proper Escape handling
  $effect(() => {
    if (isOpen) {
      overlay.focus();
    }
  });

  function handleClose() {
    isOpen = false;
    if (onClose) {
      onClose();
    }
  }

  // Optionally close the popup when clicking outside of it
  function handleClickOutside(event) {
    if (!event.target.closest('.popup-content')) {
      handleClose();
    }
  }

  /**
   * @param {KeyboardEvent} event
   */
  function onKeyOutside(event) {
    if (event.key === 'Escape') {
      event.preventDefault();
      handleClose();
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions
   background interactivity should not be user in screen readers -->
  <div
    class="overlay"
    onclick={handleClickOutside}
    onkeydown={onKeyOutside}
    tabindex="-1"
    bind:this={overlay}
  >
    <div
      class="popup-content"
      role="dialog"
      aria-modal="true"
      aria-label="pop up"
    >
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
