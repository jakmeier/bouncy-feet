<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { landmarksToKeypoints } from '$lib/pose';
  import { fileToUrl, waitForVideoMetaLoaded } from '$lib/promise_util';
  import { getContext, onMount } from 'svelte';
  import PoseError from './PoseError.svelte';

  /** @type {HTMLInputElement}  */
  let upload;
  /** @type {HTMLVideoElement}  */
  let video;

  let dataListener;
  let tracker = new Tracker();
  const poseCtx = getContext('pose');

  /**
   * @type {import("$lib/instructor/bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = [];

  async function loadVideo(event) {
    if (event.target.files && event.target.files[0]) {
      video.src = await fileToUrl(event.target.files[0]);
      await waitForVideoMetaLoaded(video);
      tracker = new Tracker();
      video.play();
      loop();
    }
  }

  function loop() {
    if (dataListener) {
      dataListener.trackFrame(video, video.currentTime * 1000);
    }
    requestAnimationFrame(loop);
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection((result, timestamp) => {
      if (result.landmarks && result.landmarks.length >= 1) {
        const kp = landmarksToKeypoints(result.landmarks[0]);
        tracker.addKeypoints(kp, timestamp);
      }
    });
  });

  function downloadFrame() {
    const keypointsRon = tracker.exportFrame(video.currentTime * 1000);

    // Create a temporary <a> to trigger the download
    const a = document.createElement('a');
    a.href =
      'data:text/plain;charset=utf-8,' + encodeURIComponent(keypointsRon);
    a.download = 'keypoints.ron';

    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
  }

  function computePoseErrors() {
    poseErrors = tracker.allPoseErrors(video.currentTime * 1000);
  }
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
<button on:click={downloadFrame}> Download Keypoints of Frame </button>
<button on:click={computePoseErrors}> Show Pose Evaluations </button>
<h2>Pose evaluations</h2>
{#each poseErrors as pose}
  <PoseError data={pose} />
{/each}
