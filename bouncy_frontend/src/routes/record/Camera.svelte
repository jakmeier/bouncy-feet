<!-- Logic to start the camera and a video element that owns the camera feed. -->
<script>
  import { onDestroy } from 'svelte';
  import { waitForVideoMetaLoaded } from '$lib/promise_util';

  /** @type HTMLVideoElement|null */
  export let videoElement = null;
  export let cameraOn = false;

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
        height: 360,
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
