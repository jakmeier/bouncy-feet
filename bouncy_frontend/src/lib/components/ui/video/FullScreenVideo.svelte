<script>
  import Symbol from '../Symbol.svelte';
  import UnstyledButton from '../UnstyledButton.svelte';
  import PeertubeVideoPlayer from './PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} videoId
   * @property {number} aspectRatio
   * @property {boolean} [isOpen]
   * @property {function} [onClose]
   */

  /** @type {Props} */
  let { isOpen = $bindable(false), onClose, videoId, aspectRatio } = $props();

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
  <div
    class="overlay"
    onkeydown={onKeyOutside}
    tabindex="-1"
    role="dialog"
    aria-modal="true"
    aria-label="Video player"
    bind:this={overlay}
  >
    <div class="close">
      <UnstyledButton onClick={() => (isOpen = false)} ariaLabel="Close video">
        <Symbol styleClass="white" size={32}>close</Symbol>
      </UnstyledButton>
    </div>

    <div class="video">
      <PeertubeVideoPlayer {videoId} {aspectRatio} timeline="inline" />
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
    background-color: var(--theme-neutral-black);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 19999;
  }

  .close {
    display: grid;
    place-items: center;
    height: 3rem;
    width: 100%;
    position: absolute;
    top: 0;
    z-index: 11;
  }

  .video {
    max-width: 100vw;
    max-height: 100vh;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
