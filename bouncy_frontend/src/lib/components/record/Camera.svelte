<!-- Logic to start the camera and a video element that owns the camera feed. -->
<script>
  import { onDestroy } from 'svelte';
  import { waitForVideoMetaLoaded } from '$lib/promise_util';
  import Symbol from '../ui/Symbol.svelte';

  /** @type HTMLVideoElement|null */
  export let videoElement = null;
  export let cameraOn = false;

  export let width = 360;
  export let height = 480;
  export let opacity = 1.0;

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
    const w = videoElement.videoWidth;
    const h = videoElement.videoHeight;
    if (w / h > width / height) {
      // video too wide, zoom to center to match height
      videoElement.style.height = height + 'px';
      const hiddenWidth = (height * w) / h;
      videoElement.style.width = hiddenWidth + 'px';
      videoElement.style['margin-left'] = `-${(hiddenWidth - width) / 2}px`;
    } else {
      // video too high, zoom to center to match width
      videoElement.style.width = width + 'px';
      const hiddenHeight = (width * h) / w;
      videoElement.style.height = hiddenHeight + 'px';
      videoElement.style['margin-top'] = `-${(hiddenHeight - height) / 2}px`;
    }

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
<video
  bind:this={videoElement}
  class:hidden={!cameraOn}
  style="opacity: {opacity}"
></video>
{#if !cameraOn}
  <Symbol size={100}>videocam</Symbol>
{/if}
