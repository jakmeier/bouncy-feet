<script>
  /** @type {string} */
  export let path;
  let showVideo = false;
  /** @type {HTMLVideoElement} */
  let videoElement;

  function toggleFullscreen() {
    if (!document.fullscreenElement) {
      videoElement.requestFullscreen().catch((err) => {
        console.error(
          `Error attempting to enable fullscreen mode: ${err.message} (${err.name})`
        );
      });
    } else {
      document.exitFullscreen();
    }
  }

  function videoExists() {
    showVideo = true;
  }
</script>

<div class="video-container {showVideo ? '' : 'hide'}">
  <video
    bind:this={videoElement}
    controls
    on:loadedmetadata={videoExists}
    preload="metadata"
  >
    <source src={path} type="video/mp4" />
    Your browser does not support the video tag.
  </video>
</div>
{#if !showVideo}
  <div class="video-unavailable">
    <span class="material-symbols-outlined"> disabled_by_default </span>
  </div>
{/if}

<style>
  .video-unavailable {
    width: 200px;
    height: 300px;
    margin: 20px auto;
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    text-align: center;
    line-height: 370px;
    border-radius: 25px;
  }
  .video-unavailable span {
    font-size: 100px;
  }

  .video-container {
    display: flex;
    width: 90%;
    height: 300px;
    margin: 20px auto;
  }

  video {
    max-width: 80vw;
    max-height: 300px;
    height: auto;
    margin: auto;
    border: 4px solid var(--theme-main);
    border-radius: 25px;
  }

  .hide {
    display: none;
  }
</style>
