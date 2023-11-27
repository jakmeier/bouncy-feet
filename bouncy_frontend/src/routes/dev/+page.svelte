<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { PoseDetection, landmarksToKeypoints } from '$lib/pose';
  import { fileToUrl, waitForVideoMetaLoaded } from '$lib/promise_util';
  import { onMount } from 'svelte';

  /** @type {HTMLInputElement}  */
  let upload;
  /** @type {HTMLVideoElement}  */
  let video;

  let dataListener;
  const tracker = new Tracker();

  async function loadVideo(event) {
    if (event.target.files && event.target.files[0]) {
      video.src = await fileToUrl(event.target.files[0]);
      await waitForVideoMetaLoaded(video);
      video.play();
      loop();
    }
  }

  function loop() {
    if (dataListener) {
      dataListener.trackFrame(video);
    }
    requestAnimationFrame(loop);
  }

  onMount(async () => {
    dataListener = await PoseDetection.new((result, timestamp) => {
      if (result.landmarks && result.landmarks.length >= 1) {
        const kp = landmarksToKeypoints(result.landmarks[0]);
        tracker.addKeypoints(kp, timestamp);
      }
    });
  });
</script>

<h1>Dev</h1>

<video bind:this={video} playsinline controls></video>
<p>
  <input
    bind:this={upload}
    type="file"
    accept="video/*"
    on:change={loadVideo}
  />
</p>
