<script>
  import Symbol from './Symbol.svelte';

  /** @type {string} */
  export let path;
  let videoExists = false;
  let videoLoading = true;
  /** @type {HTMLVideoElement} */
  let videoElement;
</script>

<div class="video-container" class:hide={videoLoading || !videoExists}>
  <div class="corner-marked">
    <video
      bind:this={videoElement}
      controls
      on:loadedmetadata={() => {
        videoExists = true;
        videoLoading = false;
      }}
      preload="metadata"
    >
      <source
        src={path}
        type="video/mp4"
        on:error={() => {
          videoLoading = false;
          videoExists = false;
        }}
        on:suspend={() => {
          videoLoading = false;
          videoExists = false;
        }}
      />
      Your browser does not support the video tag.
    </video>
  </div>
</div>

{#if videoLoading}
  <div class="video-unavailable">
    <Symbol size={100} class="rotating">refresh</Symbol>
  </div>
{:else if !videoExists}
  <div class="video-unavailable">
    <Symbol>disabled_by_default</Symbol>
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

  .video-container {
    display: flex;
    width: 90%;
    margin: 20px auto;
  }

  video {
    max-width: 80vw;
    max-height: 95vh;
    height: auto;
    margin: auto;
  }

  .hide {
    display: none;
  }
</style>
