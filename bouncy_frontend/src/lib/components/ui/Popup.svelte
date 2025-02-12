<!-- Popup.svelte -->
<script>
  import { writable } from 'svelte/store';
  import UiBox from '$lib/components/ui/UiBox.svelte';

  /** @type {import("svelte/store").Writable<boolean>} */
  export let isOpen = writable(false);
  /** @type{String|undefined} */
  export let title = undefined;
  export let showOkButton = false;

  function handleClose() {
    isOpen.set(false);
  }

  // Optionally close the popup when clicking outside of it
  function handleClickOutside(event) {
    if (!event.target.closest('.popup-content')) {
      handleClose();
    }
  }
</script>

{#if $isOpen}
  <div class="overlay" on:click={handleClickOutside}>
    <div class="popup-content">
      <UiBox {title}>
        <slot />
        {#if showOkButton}
          <button on:click={handleClose}>OK</button>
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
