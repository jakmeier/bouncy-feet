<script>
  import Camera from './Camera.svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import Canvas from '$lib/Canvas.svelte';
  import Avatar from './Avatar.svelte';
  import { getContext, onDestroy, onMount, setContext } from 'svelte';
  import Area from './Area.svelte';
  import { t } from '$lib/i18n';

  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import Banner from './Banner.svelte';

  const poseCtx = getContext('pose');

  /**
   * @type {HTMLVideoElement}
   */
  let videoElement;
  /**
   * @type {Camera}
   */
  let camera;
  /**
   * @type {undefined | import("$lib/instructor/bouncy_instructor").Skeleton}
   */
  /**
   * @type {undefined | string}
   */
  let reviewVideoSrc;
  let skeleton;
  let isModelOn = false;
  let cameraOn = false;
  let recordingStarted = false;
  let dataListener;
  let stop = false;

  /**
   * @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]}
   */
  let detectedSteps = [];

  const tracker = new Tracker();
  setContext('tracker', {
    tracker,
  });

  function loop() {
    if (isModelOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(videoElement);
      const t = performance.now() - start;
      if (t > 50) {
        console.debug(`trackFrame took ${t}ms`);
      }
      detectedSteps = tracker.detectDance();
      const t2 = performance.now() - start;
      if (t2 - t > 30) {
        console.debug(`detectDance took ${t2 - t}ms`);
      }
    }
    requestAnimationFrame(loop);
  }

  async function startCamera() {
    if (!cameraOn) {
      await camera.startCamera();
      isModelOn = true;
    }
  }

  async function startRecording() {
    await startCamera();
    camera.startRecording();
    tracker.clear();
    recordingStarted = true;
  }

  function stopCamera() {
    camera.stopCamera();
    isModelOn = false;
  }

  async function stopCameraAndRecording() {
    stopCamera();
    detectedSteps = tracker.detectDance();
    const videoBlob = await camera.endRecording();

    if (videoBlob) {
      reviewVideoSrc = URL.createObjectURL(videoBlob);
    }
  }

  function reset() {
    recordingStarted = false;
    reviewVideoSrc = undefined;
    skeleton = undefined;
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection((result, timestamp) => {
      if (result.landmarks && result.landmarks.length >= 1) {
        const kp = landmarksToKeypoints(result.landmarks[0]);
        const skeletons = tracker.addKeypoints(kp, timestamp);
        skeleton = skeletons.front;
      }
    });

    if (!stop) {
      loop();
    }
  });

  onDestroy(() => {
    stop = true;
  });
</script>

<h1>
  <!-- Space holder -->
</h1>

<div id="outer">
  <Area width="{280}px" height="{280}px">
    {#if reviewVideoSrc}
      <!-- svelte-ignore a11y-media-has-caption -->
      <video src={reviewVideoSrc} playsinline controls></video>
    {:else}
      <Camera bind:videoElement bind:cameraOn bind:this={camera} />
    {/if}
  </Area>

  <Area width="{280}px" height="{280}px">
    <Canvas width={300} height={300}>
      <Avatar width={300} height={300} {skeleton} />
    </Canvas>
  </Area>

  {#if recordingStarted}
    <Banner steps={detectedSteps}></Banner>
  {/if}

  <div>
    {#if !cameraOn && !recordingStarted}
      <button on:click={startCamera}>
        <span class="material-symbols-outlined"> videocam </span>
        <p>{$t('record.start-button')}</p>
      </button>
    {:else if !recordingStarted}
      <button on:click={stopCamera}>
        <span class="material-symbols-outlined"> videocam_off </span>
        <p>{$t('record.stop-button')}</p>
      </button>
    {/if}

    {#if !recordingStarted}
      <button on:click={startRecording}>
        <span class="material-symbols-outlined"> radio_button_unchecked </span>
        <p>{$t('record.record-button')}</p>
      </button>
    {:else if isModelOn}
      <button on:click={stopCameraAndRecording}>
        <span class="material-symbols-outlined"> camera </span>
        <p>{$t('record.stop-button')}</p>
      </button>
    {:else}
      <button on:click={reset}>
        <span class="material-symbols-outlined"> done </span>
        <p>{$t('record.done-button')}</p>
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

  button {
    width: 152px;
    height: 80px;
    margin: 10px;
  }
  button span {
    font-size: 42px;
  }

  @media (max-width: 360px) {
    /* Two buttons must fit next to each other, including margin */
    button {
      width: 120px;
    }
  }
</style>
