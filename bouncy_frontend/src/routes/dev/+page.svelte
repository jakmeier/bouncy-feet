<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { landmarksToKeypoints } from '$lib/pose';
  import { fileToUrl, waitForVideoMetaLoaded } from '$lib/promise_util';
  import { getContext, onMount, setContext } from 'svelte';
  import PoseError from '$lib/components/dev/PoseError.svelte';
  import Banner from '$lib/components/review/Banner.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';

  /** @type {HTMLInputElement}  */
  let upload;
  /** @type {HTMLVideoElement}  */
  let video;

  let dataListener;
  let tracker = new Tracker();
  setContext('tracker', {
    tracker,
  });
  const poseCtx = getContext('pose');

  /** @type {undefined | number} */
  let recordingStart;
  /** @type {undefined | number} */
  let recordingEnd;

  /**
   * @type {import("$lib/instructor/bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = [];
  /**
   * @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]}
   */
  let detectedSteps = [];

  async function loadVideo(event) {
    if (event.target.files && event.target.files[0]) {
      video.src = await fileToUrl(event.target.files[0]);
      await waitForVideoMetaLoaded(video);
      tracker.clear();
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
      if (recordingStart === undefined) {
        recordingStart = timestamp;
      }
      if (result.landmarks && result.landmarks.length >= 1) {
        const kp = landmarksToKeypoints(result.landmarks[0]);
        tracker.addKeypoints(kp, BigInt(timestamp));
        recordingEnd = timestamp;
      }
    });
  });

  /**
   * @param {string} filename
   * @param {string} text
   */
  function downloadTextFile(filename, text) {
    // Create a temporary <a> to trigger the download
    const a = document.createElement('a');
    a.href = 'data:text/plain;charset=utf-8,' + encodeURIComponent(text);
    a.download = filename;

    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
  }

  function downloadFrame() {
    const exported = tracker.exportFrame(BigInt(video.currentTime * 1000));
    downloadTextFile('keypoints.ron', exported.keypoints);
    downloadTextFile('pose.ron', exported.pose);
  }

  function downloadKeypoints() {
    const exported = tracker.exportKeypoints();
    downloadTextFile('keypoints.ron', exported);
  }

  function computePoseErrors() {
    poseErrors = tracker.allPoseErrors(BigInt(video.currentTime * 1000));
  }

  function logDance() {
    detectedSteps = tracker.detectDance().steps();
    detectedSteps.forEach((step) => {
      console.log(step.name, step.start, step.end);
    });
  }
</script>

<h1>Dev</h1>

<!-- svelte-ignore a11y-media-has-caption -->
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
<button on:click={downloadKeypoints}> Download Keypoints of Video </button>
<h2>Dance Evaluation</h2>
<button on:click={logDance}> Log Dance </button>
{#if detectedSteps.length > 0 && video}
  <VideoReview
    reviewVideoSrc={video.src}
    {detectedSteps}
    {recordingStart}
    {recordingEnd}
  ></VideoReview>
{/if}
<h2>Pose evaluations</h2>
<button on:click={computePoseErrors}> Show Pose Evaluations </button>
{#each poseErrors as pose}
  <PoseError data={pose} />
{/each}

<style>
  video {
    max-width: 100%;
  }
</style>
