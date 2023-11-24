<script>
  import Camera from './Camera.svelte';
  import { PoseDetection, I } from '$lib/pose';
  import Canvas from '$lib/Canvas.svelte';
  import Avatar from './Avatar.svelte';
  import { onDestroy, onMount } from 'svelte';
  import Area from './Area.svelte';
  import { t } from '$lib/i18n';

  import {
    Tracker,
    Keypoints,
    KeypointsSide,
    Coordinate3d,
  } from '$lib/instructor/bouncy_instructor';

  let videoElement;
  let camera;
  let landmarks = [];
  let isModelOn = false;
  let cameraOn = false;
  let dataListener;
  let stop = false;

  const tracker = new Tracker();

  function loop() {
    if (isModelOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(videoElement);
      const t = performance.now() - start;
      if (t > 20) {
        console.debug(`loop trackFrame took ${t}ms`);
      }
    }
    requestAnimationFrame(loop);
  }

  /**
   *
   * @param {number} i
   * @param {import('@mediapipe/tasks-vision').Landmark[]} landmarks
   */
  function coordinate(i, landmarks) {
    return new Coordinate3d(landmarks[i].x, landmarks[i].y, landmarks[i].z);
  }

  async function startCamera() {
    await camera.startCamera();
    isModelOn = true;
  }

  function stopCamera() {
    camera.stopCamera();
    isModelOn = false;
  }

  onMount(async () => {
    dataListener = await PoseDetection.new((result, timestamp) => {
      if (result.landmarks && result.landmarks.length >= 1) {
        landmarks = result.landmarks[0];
        // landmarks = result.worldLandmarks[0];

        const data = result.landmarks[0];
        const left = new KeypointsSide(
          coordinate(I.LEFT_SHOULDER, data),
          coordinate(I.LEFT_HIP, data),
          coordinate(I.LEFT_KNEE, data),
          coordinate(I.LEFT_ANKLE, data)
        );

        const right = new KeypointsSide(
          coordinate(I.RIGHT_SHOULDER, data),
          coordinate(I.RIGHT_HIP, data),
          coordinate(I.RIGHT_KNEE, data),
          coordinate(I.RIGHT_ANKLE, data)
        );
        tracker.add_keypoints(new Keypoints(left, right), timestamp);
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

<div id="outer">
  <Area width="{280}px" height="{280}px">
    <Camera bind:videoElement bind:cameraOn bind:this={camera} />
  </Area>
  <Area width="{280}px" height="{280}px">
    <Canvas width={300} height={300}>
      <Avatar {landmarks} />
    </Canvas>
  </Area>
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

  <p>[recording settings]</p>
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
</style>
