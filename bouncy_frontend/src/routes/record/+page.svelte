<script>
  import Camera from './Camera.svelte';
  import { PoseDetection } from '$lib/pose';
  import Canvas from '$lib/Canvas.svelte';
  import Avatar from './Avatar.svelte';
  import { onDestroy, onMount } from 'svelte';

  let videoElement;
  let landmarks = [];
  let isModelOn = false;
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

<Camera bind:videoElement />
<button on:click={startModel}>TEST: start model</button>
<button on:click={stopModel}>TEST: stop model</button>
<Canvas width="{300}," height={300}>
  <Avatar {landmarks} />
</Canvas>
<p>[recording settings]</p>
