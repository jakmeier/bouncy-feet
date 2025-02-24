<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import { registerTracker } from '$lib/stores/Beat';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar2 from '$lib/components/avatar/SvgAvatar2.svelte';
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import PoseAnglesForm from '$lib/components/editor/PoseAnglesForm.svelte';
  import PoseWeightsForm from '$lib/components/editor/PoseWeightsForm.svelte';
  import { fileToUrl, waitForVideoMetaLoaded } from '$lib/promise_util';
  import { PoseDetection } from '$lib/pose';
  import Button from '../ui/Button.svelte';

  const poseCtx = getContext('pose');
  const localCollectionCtx = getContext('localCollection');
  let tracker = new Tracker();
  registerTracker(tracker);

  /** @type {PoseDetection} */
  let dataListener;
  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").SkeletonWrapper)=>void} */
  let loadSkeleton = $state();
  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let poseFromForm = $state();
  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").PoseWrapper)=>void} */
  let loadPose = $state();
  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let getPose = $state();

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonV2 | undefined} */
  let liveSkeleton = $state();
  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonWrapper | undefined} */
  let poseSkeleton;

  /** @type {HTMLInputElement}  */
  let upload = $state();
  /** @type {HTMLVideoElement}  */
  let video = $state();
  let videoSrcWidth = $state(0);
  let videoSrcHeight = $state(0);

  /** @type {undefined | number} */
  let recordingStart;
  /** @type {undefined | number} */
  let recordingEnd;
  let prevTime = -1;
  let selectedTimestamp = 0;

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
      dataListener.trackFrame(video, undefined);
    }
    requestAnimationFrame(loop);
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(
      (
        /** @type {{ landmarks: string | any[]; }} */ result,
        /** @type {number} */ timestamp
      ) => {
        if (recordingStart === undefined) {
          recordingStart = timestamp;
        }
        if (result.landmarks && result.landmarks.length >= 1) {
          const kp = landmarksToKeypoints(result.landmarks[0]);
          tracker.addKeypoints(kp, timestamp);
          recordingEnd = Math.max(timestamp, recordingEnd || 0);
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

  function copySkeleton() {
    poseSkeleton = tracker.skeletonWrapperAt(selectedTimestamp);
    if (poseSkeleton) {
      loadSkeleton(poseSkeleton);
    }
  }

  function copyPose() {
    let pose = poseFromForm();
    if (pose) {
      loadPose(pose);
    }
  }

  function savePose() {
    let pose = getPose();
    localCollectionCtx.addPose(pose);
  }
</script>

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
        <SvgAvatar2
          skeleton={liveSkeleton}
          lineWidth={3}
          style={LEFT_RIGHT_COLORING_LIGHT}
        />
      </Svg>
    {/if}
  </div>
</div>

<button class="full-width short" onclick={copySkeleton}> ↓ </button>

<PoseAnglesForm bind:loadSkeleton bind:readPose={poseFromForm}></PoseAnglesForm>

<button class="full-width short" onclick={copyPose}> ↓ </button>

<PoseWeightsForm bind:loadPose bind:getPose></PoseWeightsForm>

<Button
  symbol="save"
  symbolSize={29}
  class="full-width short"
  on:click={savePose}
></Button>

<style>
  video {
    max-width: 100%;
  }

  .side-by-side {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }
</style>
