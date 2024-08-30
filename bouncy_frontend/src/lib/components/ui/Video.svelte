<script>
  /** @type {string} */
  export let path;
  let videoReady = false;
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
</script>

{#if videoReady}
  <div class="video-mock">
    <span class="material-symbols-outlined"> play_arrow </span>
  </div>
{:else}
  <div class="video-container">
    <video bind:this={videoElement} controls>
      <source src={path} type="video/mp4" />
      Your browser does not support the video tag.
    </video>
  </div>
{/if}

<style>
  .video-mock {
    width: 90%;
    height: 300px;
    margin: 20px auto;
    background-color: black;
    color: whitesmoke;
    text-align: center;
    line-height: 370px;
  }
  .video-mock span {
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
    max-height: 70vw;
    height: auto;
    margin: auto;
    border: 1px solid #ccc;
  }
</style>
