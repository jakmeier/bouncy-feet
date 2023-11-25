<!-- Logic to start the camera and a video element that owns the camera feed. -->
<script>
  import { onDestroy } from 'svelte';
  import Area from './Area.svelte';
  import { waitForVideoMetaLoaded } from '$lib/promise_util';

  /** @type HTMLVideoElement|null */
  export let videoElement = null;
  export let cameraOn = false;

  export async function startCamera() {
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

    await waitForVideoMetaLoaded(videoElement);
    videoElement.play();

    cameraOn = true;
  }

  export function stopCamera() {
    if (videoElement && videoElement.srcObject) {
      videoElement.srcObject.getTracks().forEach((track) => {
        track.stop();
      });
    }
    cameraOn = false;
  }

  onDestroy(stopCamera);
</script>

<!-- svelte-ignore a11y-media-has-caption -->
<video bind:this={videoElement} class:hidden={!cameraOn}></video>
{#if !cameraOn}
  <span class="material-symbols-outlined"> videocam </span>
{/if}

<style>
  span {
    font-size: 100px;
  }
</style>
