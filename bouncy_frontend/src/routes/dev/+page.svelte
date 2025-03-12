<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { landmarksToKeypoints } from '$lib/pose';
  import { fileToUrl, waitForVideoMetaLoaded } from '$lib/promise_util';
  import { getContext, onMount } from 'svelte';
  import PoseError from '$lib/components/dev/PoseError.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import { registerTracker } from '$lib/stores/Beat';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar2 from '$lib/components/avatar/SvgAvatar2.svelte';
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { downloadTextFile } from '$lib/text_utils';

  /** @type {HTMLInputElement}  */
  let upload = $state();
  /** @type {HTMLVideoElement}  */
  let video = $state();
  let prevTime = -1;
  let selectedTimestamp = 0;
  let videoSrcWidth = $state(0);
  let videoSrcHeight = $state(0);
  /** @type {import("bouncy_instructor").SkeletonV2 | undefined} */
  let liveSkeleton = $state();

  let dataListener;
  /** @type {(skeleton: import("bouncy_instructor").SkeletonWrapper)=>void} */
  let loadSkeleton;
  /** @type {()=>import("bouncy_instructor").PoseWrapper} */
  let poseFromForm;
  /** @type {(skeleton: import("bouncy_instructor").PoseWrapper)=>void} */
  let loadPose;
  let tracker = new Tracker();
  registerTracker(tracker);
  const poseCtx = getContext('pose');

  /** @type {undefined | number} */
  let recordingStart = $state();
  /** @type {undefined | number} */
  let recordingEnd = $state();

  /**
   * @type {import("bouncy_instructor").PoseApproximation[]}
   */
  let poseErrors = $state([]);
  /**
   * @type {import("bouncy_instructor").DetectedStep[]}
   */
  let detectedSteps = $state([]);

  /** @param { Event } event */
  async function loadVideo(event) {
    if (event.target && event.target.files && event.target.files[0]) {
      video.src = await fileToUrl(event.target.files[0]);
      await waitForVideoMetaLoaded(video);
      tracker.clear();
      video.play();
      loop();
    }
  }

  function loop() {
    if (dataListener && prevTime !== video.currentTime) {
      prevTime = video.currentTime;
      dataListener.trackFrame(video);
      // dataListener.trackFrame(video, video.currentTime * 1000);
    }
    requestAnimationFrame(loop);
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(
      (
        /** @type {{ landmarks: string | any[]; }} */ result,
        /** @type {number | undefined} */ timestamp
      ) => {
        if (recordingStart === undefined) {
          recordingStart = timestamp;
        }
        if (result.landmarks && result.landmarks.length >= 1) {
          const kp = landmarksToKeypoints(result.landmarks[0]);
          tracker.addKeypoints(kp, timestamp);
          recordingEnd = Math.max(timestamp, recordingEnd);
          selectedTimestamp = timestamp;
          liveSkeleton = tracker.renderedKeypointsAt(
            timestamp,
            videoSrcWidth,
            videoSrcHeight
          );
        }
      }
    );
  });

  function downloadFrame() {
    const exported = tracker.exportFrame(video.currentTime * 1000);
    downloadTextFile('keypoints.ron', exported.keypoints);
    downloadTextFile('pose.ron', exported.pose);
  }

  function downloadKeypoints() {
    const exported = tracker.exportKeypoints();
    downloadTextFile('keypoints.ron', exported);
  }

  function computePoseErrors() {
    poseErrors = tracker.allPoseErrors(video.currentTime * 1000);
  }

  function logDance() {
    detectedSteps = tracker.detectDance().steps();
    detectedSteps.forEach((step) => {
      console.log(step.name, step.start, step.end);
    });
  }
</script>

<h1>Dev</h1>

<p>
  <input
    bind:this={upload}
    type="file"
    accept="video/*"
    onchange={loadVideo}
  />
</p>

<!-- svelte-ignore a11y_media_has_caption -->
<div class="side-by-side">
  <video
    bind:this={video}
    bind:videoWidth={videoSrcWidth}
    bind:videoHeight={videoSrcHeight}
    playsinline
    controls
  ></video>
  <div>
    {#if liveSkeleton}
      <Svg width={videoSrcWidth} height={videoSrcHeight} orderByZ showOverflow>
        <SvgAvatar2 skeleton={liveSkeleton} />
      </Svg>
    {/if}
  </div>
</div>

<button onclick={downloadFrame}> Download Keypoints of Frame </button>
<button onclick={downloadKeypoints}> Download Keypoints of Video </button>
<h2>Dance Evaluation</h2>
<button onclick={logDance}> Log Dance </button>
{#if detectedSteps.length > 0 && video}
  <VideoReview
    reviewVideoSrc={video.src}
    {detectedSteps}
    {recordingStart}
    {recordingEnd}
  ></VideoReview>
{/if}
<h2>Pose evaluations</h2>
<button onclick={computePoseErrors}> Show Pose Evaluations </button>
{#each poseErrors as pose}
  <PoseError data={pose} />
{/each}

<style>
  video {
    max-width: 100%;
  }

  .side-by-side {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }
</style>
