<!-- Logic to start the camera and a video element that owns the camera feed. -->
<script>
  import { onDestroy } from 'svelte';
  import { waitForVideoMetaLoaded } from '$lib/promise_util';
  import { selectMediaMimeType } from '$lib/media';

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
    opacity = $bindable(1.0),
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
  let videoMimeType = selectMediaMimeType();

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
    if (videoMimeType) {
      try {
        recorder = new MediaRecorder(stream, { mimeType: videoMimeType });
      } catch (err) {
        console.warn(
          'Failed to create MediaRecorder with mimeType',
          videoMimeType,
          err
        );
      }
    }

    if (!recorder) {
      // fallback: probably it wont't work to record but we can try anyway,
      // letting the browser use whatever codec it wants.
      recorder = new MediaRecorder(stream);
    }

    recordedBlobs = [];
    recorder.ondataavailable = (event) => {
      if (event.data && event.data.size > 0) {
        recordedBlobs.push(event.data);
      }
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
      // Create promise to wait for the stop event to resolve
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

      // Safari likes to get here before data was sent, even when awaiting the stop event.
      // Not sure what the best way is to make it work but this is one way of doing it.
      let attempts = 100;
      while (attempts-- && recordedBlobs.length === 0) {
        await new Promise((resolve) => setTimeout(resolve, 20));
      }

      if (recordedBlobs.length === 0) {
        console.warn('failed recording with mime type', videoMimeType);
        return null;
      }

      if (videoMimeType) {
        return new Blob(recordedBlobs, { type: videoMimeType });
      } else {
        return new Blob(recordedBlobs);
      }
    }
  }

  onDestroy(stopCamera);
</script>

<!-- svelte-ignore a11y_media_has_caption -->
<video
  bind:this={videoElement}
  class:hidden={!cameraOn}
  style="opacity: {opacity}"
  playsinline
  webkit-playsinline
></video>

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
    transition: opacity 0.5s ease-in-out;
  }
</style>
