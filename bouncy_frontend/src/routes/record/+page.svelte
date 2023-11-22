<script>
  import Camera from './Camera.svelte';
  import { PoseDetection } from '$lib/pose';
  import Canvas from '$lib/Canvas.svelte';
  import Avatar from './Avatar.svelte';
  import { onDestroy, onMount } from 'svelte';
  import Area from './Area.svelte';
  import { t } from '$lib/i18n';

  let videoElement;
  let camera;
  let landmarks = [];
  let isModelOn = false;
  let cameraOn = false;
  let renderer;
  let stop = false;

  async function startModel() {
    isModelOn = true;
  }

  function stopModel() {
    isModelOn = false;
  }

  function loop() {
    if (isModelOn && renderer) {
      renderer.trackVideoFrame(videoElement);
    }
    requestAnimationFrame(loop);
  }

  onMount(async () => {
    renderer = await PoseDetection.new((result) => {
      if (result.landmarks && result.landmarks.length >= 1) {
        landmarks = result.landmarks[0];
        // landmarks = result.worldLandmarks[0];
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
  <Area width={280}px height={280}px>
    <Camera bind:videoElement bind:cameraOn bind:this={camera}/>
  </Area>
  <Area width={280}px height={280}px>
    <Canvas width={300} height={400}>
      <Avatar {landmarks} />
    </Canvas>
  </Area>
  <div>
    {#if cameraOn}
      <button on:click={camera.stopCamera}>
        <span class="material-symbols-outlined"> stop </span>
        <p>{$t('record.stop-button')}</p>
      </button>
    {:else}
      <button on:click={camera.startCamera}>
        <span class="material-symbols-outlined"> radio_button_unchecked </span>
        <p>{$t('record.start-button')}</p>
      </button>
    {/if}
  </div>

  <button on:click={startModel}>TEST: start model</button>
  <button on:click={stopModel}>TEST: stop model</button>

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
    height: 95px;
  }
  button span {
    font-size: 55px;
  }
</style>
