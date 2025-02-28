<script>
  import Symbol from './Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} path
   * @property {boolean} [controls=true]
   */

  /** @type {Props} */
  let { path, controls = true } = $props();
  let videoExists = $state(false);
  let videoLoading = $state(true);
  /** @type {HTMLVideoElement} */
  let videoElement = $state();

  export function startVideo() {
    if (videoElement) {
      videoElement.play();
    }
  }
</script>

<div class="video-container" class:hide={videoLoading || !videoExists}>
  <div class="corner-marked">
    <video
      bind:this={videoElement}
      {controls}
      onloadedmetadata={() => {
        videoExists = true;
        videoLoading = false;
      }}
      preload="auto"
    >
      <source
        src={path}
        type="video/mp4"
        onerror={() => {
          videoLoading = false;
          videoExists = false;
        }}
        onsuspend={() => {
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
  }

  video {
    max-width: 80vw;
    max-height: 95dvh;
    height: auto;
    margin: auto;
  }

  .hide {
    display: none;
  }
</style>
