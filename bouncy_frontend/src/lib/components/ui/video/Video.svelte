<script>
  import CornerMarker from '../CornerMarker.svelte';
  import Symbol from '../Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} path
   * @property {boolean} [controls=true]
   * @property {boolean} [muted=false]
   */

  /** @type {Props} */
  let { path, controls = true, muted = false } = $props();
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
  <CornerMarker>
    <div class="inner-video-container">
      <video
        bind:this={videoElement}
        {controls}
        onloadedmetadata={() => {
          videoExists = true;
          videoLoading = false;
        }}
        preload="auto"
        playsinline
        webkit-playsinline
        defaultmuted={muted}
        {muted}
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
  </CornerMarker>
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
  .video-container {
    position: relative;
    display: flex;
    width: 100%;
    height: 100%;
  }

  .inner-video-container {
    width: 100%;
    height: 100%;
  }

  .inner-video-container {
    position: relative;
    overflow: hidden;
  }

  video {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .hide {
    display: none;
  }

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
</style>
