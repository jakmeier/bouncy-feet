<!-- User controls to start the camera and a video element that owns the camera feed. -->
<script>
  import { t } from '$lib/i18n';
  import { onDestroy } from 'svelte';

  export let size = '250px';
  /** @type HTMLVideoElement|null */
  export let videoElement = null;
  let cameraOn = false;

  async function startCamera() {
    if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
      throw new Error(
        'Browser API navigator.mediaDevices.getUserMedia not available'
      );
    }

    const videoConfig = {
      audio: false,
      video: {
        facingMode: 'user',
        width: 360,
        height: 360,
        frameRate: {
          ideal: 60,
        },
      },
    };

    videoElement.srcObject =
      await navigator.mediaDevices.getUserMedia(videoConfig);

    await new Promise((resolve) => {
      videoElement.onloadedmetadata = () => {
        resolve(videoElement);
      };
    });
    videoElement.play();

    cameraOn = true;
  }

  function stopCamera() {
    if (videoElement && videoElement.srcObject) {
      videoElement.srcObject.getTracks().forEach((track) => {
        track.stop();
      });
    }
    cameraOn = false;
  }

  onDestroy(stopCamera);
</script>

<div id="outer">
  <div class="cam-container" style="min-height: {size}; min-width: {size};">
    <!-- svelte-ignore a11y-media-has-caption -->
    <video bind:this={videoElement} class:hidden={!cameraOn}></video>
    {#if !cameraOn}
      <span class="material-symbols-outlined"> videocam </span>
    {/if}
  </div>
  <div>
    {#if cameraOn}
      <button on:click={stopCamera}>
        <span class="material-symbols-outlined"> stop </span>
        <p>{$t('record.stop-button')}</p>
      </button>
    {:else}
      <button on:click={startCamera}>
        <span class="material-symbols-outlined"> radio_button_unchecked </span>
        <p>{$t('record.start-button')}</p>
      </button>
    {/if}
  </div>
</div>

<style>
  #outer {
    margin: auto;
    display: grid;
    justify-items: center;
  }

  .cam-container {
    display: grid;
    align-items: center;
    justify-items: center;
    margin: 10px auto;
    border: 5px var(--theme-neutral-light) solid;
    border-radius: 100px;
    overflow: hidden;
  }
  .cam-container span {
    font-size: 100px;
  }

  button {
    width: 152px;
    height: 95px;
  }
  button span {
    font-size: 55px;
  }
</style>
