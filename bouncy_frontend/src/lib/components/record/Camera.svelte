<!-- Logic to start the camera and a video element that owns the camera feed. -->
<script>
  import { onDestroy } from 'svelte';
  import { waitForVideoMetaLoaded } from '$lib/promise_util';
  import Symbol from '../ui/Symbol.svelte';
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {any} [videoElement]
   * @property {boolean} [cameraOn]
   * @property {number} [opacity]
   */

  /** @type {Props} */
  let {
    videoElement = $bindable(),
    cameraOn = $bindable(false),
    opacity = 1.0,
  } = $props();

  let stream;
  /**
   * @type {MediaRecorder | null}
   */
  let recorder = null;
  /**
   * @type {BlobPart[]}
   */
  let recordedBlobs = [];

  export async function startCamera() {
    if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
      throw new Error(
        'Browser API navigator.mediaDevices.getUserMedia not available'
      );
    }
    if (!videoElement) {
      throw new Error('Missing video element for camera');
    }

    const videoConfig = {
      audio: false,
      video: {
        facingMode: 'user',
        width: 360,
        height: 480,
        // The only way I managed to get a portrait camera on phones...
        aspectRatio: { exact: 1.5 },
        frameRate: {
          ideal: 60,
        },
      },
    };

    stream = await navigator.mediaDevices.getUserMedia(videoConfig);
    videoElement.srcObject = stream;
    await waitForVideoMetaLoaded(videoElement);
    videoElement.play();

    cameraOn = true;
  }

  export function startRecording() {
    recorder = new MediaRecorder(stream);
    recordedBlobs = [];
    recorder.ondataavailable = (event) => {
      recordedBlobs.push(event.data);
    };
    recorder.onerror = (e) => console.log(`recorder error: ${e}`);
    recorder.start();
  }

  export function stopCamera() {
    if (videoElement && videoElement.srcObject) {
      videoElement.srcObject.getTracks().forEach((track) => {
        track.stop();
      });
    }
    cameraOn = false;
  }

  export async function endRecording() {
    if (recorder) {
      let stopped = new Promise((resolve, reject) => {
        if (recorder) {
          recorder.onstop = resolve;
          recorder.onerror = (event) => reject(event);
        } else {
          resolve(null);
        }
      });
      recorder.stop();
      await stopped.catch((e) => console.log(`stopping recorder error: ${e}`));

      return new Blob(recordedBlobs, { type: 'video/webm' });
    }
  }

  onDestroy(stopCamera);
</script>

<!-- svelte-ignore a11y_media_has_caption -->
<video
  bind:this={videoElement}
  class:hidden={!cameraOn}
  style="opacity: {opacity}"
></video>
{#if !cameraOn}
  <div class="symbol corner-marked">
    <img src="{base}/img/symbols/bf_eye.svg" alt="bouncy feet eye" />
  </div>
{/if}

<style>
  video {
    /* not working as expected */
    /* object-fit: cover; */
    /* width: 100%; */
    /* height: 100%; */

    position: absolute;
    top: 50%;
    left: 50%;
    min-width: 100vw;
    min-height: 100dvh;
    transform: translate(-50%, -50%);
  }

  .symbol {
    max-width: 500px;
    min-width: 10rem;
    padding: 5rem;
  }
</style>
