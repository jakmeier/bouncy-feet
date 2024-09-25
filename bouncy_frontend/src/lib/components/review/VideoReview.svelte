<script>
  import { dev } from '$lib/stores/FeatureSelection.js';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { getContext, onMount } from 'svelte';
  import BackgroundTask from '$lib/components/BackgroundTask.svelte';
  import PoseReview from './PoseReview.svelte';
  import {
    LEFT_RIGHT_COLORING_LIGHT,
    MAIN_THEME_COLORING,
  } from '$lib/constants';
  import {
    LimbError,
    PoseApproximation,
  } from '$lib/instructor/bouncy_instructor';
  import SvgAvatar2 from '../avatar/SvgAvatar2.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import { t } from '$lib/i18n';

  /** @type {string} URL (usually local) to the video for review  */
  export let reviewVideoSrc;
  /** @type {number} */
  export let recordingStart;
  /** @type {number} */
  export let recordingEnd;
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let detectedSteps = [];

  let videoSrcWidth = 0;
  let videoSrcHeight = 0;
  let videoLoaded = false;
  let displayVideoOverlay = true;

  $: firstPoseTime = detectedSteps.length > 0 ? detectedSteps[0].start : 0;
  $: lastPoseTime =
    detectedSteps.length > 0
      ? detectedSteps[detectedSteps.length - 1].end
      : 100;

  let tracker = getContext('tracker').tracker;
  const rightAvatarLineWidth = 5;

  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  let skeleton;
  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonV2} */
  let keypointSkeleton;
  /** @type {LimbError[]} */
  let limbErrors = [];
  let selectedStep = -1;
  let selectedBeat = -1;
  $: beatsPerStep =
    detectedSteps.length > 0 ? detectedSteps[0].poses.length : 4;
  $: avatarSizePixels = videoSrcHeight;
  $: headRadius = 0.075 * Math.min(videoSrcHeight, videoSrcWidth);
  let markedSegments = [];
  $: if (keypointSkeleton) {
    markedSegments = limbErrors.map((limb) => limb.render(keypointSkeleton));
  }

  let prevTime = 0;
  function onFrame() {
    if (prevTime !== reviewVideoElement.currentTime) {
      onSeek();
    }
    prevTime = reviewVideoElement.currentTime;
  }

  function onSeek() {
    const ms = reviewVideoElement.currentTime * 1000;
    const reviewTimestamp = ms + recordingStart;
    skeleton = tracker.skeletonAt(reviewTimestamp);
    if (videoLoaded) {
      keypointSkeleton = tracker.renderedKeypointsAt(
        reviewTimestamp,
        videoSrcWidth,
        videoSrcHeight
      );
    }
    const cursor = ms / (recordingEnd - recordingStart);
    if (setCursor) {
      setCursor(cursor);
    }
  }

  /**
   * Used to communicate a cursor seek on the banner to the video.
   *
   * Manually called by child banner. Due to cyclic reactivity, it seems easier
   * than using reactive statements (but maybe I just don't know how to use them
   * properly in such cases)
   * @param {number} cursor
   */
  function seekVideoToCursor(cursor) {
    if (reviewVideoElement.paused) {
      reviewVideoElement.currentTime =
        (cursor * (recordingEnd - recordingStart)) / 1000;
    }
  }

  // Communicate a cursor seek from the video to the banner.
  /**
   * @type {(cursor: number) => void}
   */
  let setCursor;

  // Set the position in the video and the banner to the specified recording timestamp.
  /** @param {number} timestamp */
  function selectTimestamp(timestamp) {
    const videoTime = (Number(timestamp) - recordingStart) / 1000;
    reviewVideoElement.currentTime = videoTime;
  }

  /** @returns {null | PoseApproximation}*/
  function selectedPose() {
    if (selectedBeat < 0) {
      return null;
    }
    if (
      detectedSteps &&
      detectedSteps.length > selectedStep &&
      detectedSteps[selectedStep].poses.length > selectedBeat
    ) {
      return detectedSteps[selectedStep].poses[selectedBeat];
    } else {
      console.error('beat out of range', selectedStep, selectedBeat);
      return null;
    }
  }

  // Load the specified beat to the review cursor.
  /**
   * @param {number} step
   * @param {number} beat
   * */
  function selectBeat(step, beat) {
    selectedStep = step;
    selectedBeat = beat;
    const pose = selectedPose();
    if (!pose) {
      return;
    }
    selectTimestamp(pose.timestamp);
    // TODO: deduplicate threshold definition
    const threshold = 0.05;
    if (pose.error >= threshold) {
      limbErrors = pose
        .worstLimbs(6)
        .filter(
          (pose) =>
            pose.error * pose.weight > threshold && pose.name.length < 30
        );
    } else {
      limbErrors = [];
    }
  }

  // select a pose if the video time is right
  /** @param {number} t */
  function checkTimeOnBeat(t) {
    // find the last step that didn't end before t
    selectedStep = 0;
    while (
      detectedSteps &&
      detectedSteps.length > selectedStep &&
      t > detectedSteps[selectedStep].end
    ) {
      selectedStep++;
    }
    // find the last pose that was before t - maxDt
    const maxDt = 50;
    selectedBeat = 0;
    while (
      detectedSteps[selectedStep] &&
      detectedSteps[selectedStep].poses.length > selectedBeat &&
      t - maxDt > detectedSteps[selectedStep].poses[selectedBeat].timestamp
    ) {
      selectedBeat++;
    }
    const pose = selectedPose();
    if (!pose || Math.abs(pose.timestamp - t) > maxDt) {
      selectedBeat = -1;
      limbErrors = [];
    }
  }

  function togglePlay() {
    if (reviewVideoElement.paused) {
      reviewVideoElement.play();
      limbErrors = [];
    } else {
      reviewVideoElement.pause();
    }
  }

  /** @type {HTMLDivElement} */
  let poseCards;
  function onScrollPoseCards() {
    const r =
      poseCards.scrollLeft / (poseCards.scrollWidth - poseCards.clientWidth);
    const t = firstPoseTime + (lastPoseTime - firstPoseTime) * r;
    selectTimestamp(t);
    checkTimeOnBeat(t);
  }

  /**
   * @param {number} beat
   * @param {number} step
   * @param {number} stepLen
   */
  function formatBeatLabel(beat, step, stepLen) {
    if (beat < 0) {
      return '?';
    }
    const i = step * stepLen + beat;
    return `${i + 1}`;
  }

  onMount(() => {
    selectBeat(0, 0);
  });
</script>

<!-- update banner position on every frame, to keep it in sync with the video
even when it is playing. In theory, `on:timeupdate={onSeek}` in the video would
be better, however, this fires at different rates per browser and often only
once per 250ms. -->
<BackgroundTask {onFrame}></BackgroundTask>

<div class="poses-details" bind:this={poseCards} on:scroll={onScrollPoseCards}>
  {#each detectedSteps as step, iStep}
    {#each step.poses as pose, iBeat}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <!-- TODO(performance): step.poses.length allocates a vector and an array every time it is read -->
      <div on:click={() => selectBeat(iStep, iBeat)}>
        <PoseReview
          {pose}
          beatLabel={formatBeatLabel(iBeat, iStep, step.poses.length)}
        ></PoseReview>
      </div>
    {/each}
  {/each}
</div>

<div class="background-strip">
  <div class="beat-label">
    {formatBeatLabel(selectedBeat, selectedStep, beatsPerStep)}
  </div>
  <div class="upper">
    <div class="video-wrapper">
      <!-- svelte-ignore a11y-media-has-caption -->
      <video
        on:click={togglePlay}
        on:loadeddata={() => {
          videoLoaded = true;
          onSeek();
        }}
        bind:this={reviewVideoElement}
        bind:videoWidth={videoSrcWidth}
        bind:videoHeight={videoSrcHeight}
        src={reviewVideoSrc}
        playsinline
        style="max-width: 100%"
      ></video>
      {#if keypointSkeleton && displayVideoOverlay}
        <div class="video-overlay" style="pointer-events: none;">
          <Svg
            width={videoSrcWidth}
            height={videoSrcHeight}
            orderByZ
            showOverflow
          >
            <SvgAvatar2
              skeleton={keypointSkeleton}
              {avatarSizePixels}
              lineWidth={3}
              style={LEFT_RIGHT_COLORING_LIGHT}
              {headRadius}
              {markedSegments}
            />
          </Svg>
        </div>
      {/if}
    </div>

    {#if keypointSkeleton}
      <div>
        <Svg
          height={videoSrcHeight}
          width={videoSrcWidth}
          orderByZ
          showOverflow
        >
          <SvgAvatar2
            lineWidth={rightAvatarLineWidth}
            skeleton={keypointSkeleton}
            {avatarSizePixels}
            style={MAIN_THEME_COLORING}
            {headRadius}
            {markedSegments}
          />
        </Svg>
      </div>
    {/if}
    <div class="toggle-item">
      <div>{$t('record.settings.enable-tracking')}</div>
      <Toggle bind:isOn={displayVideoOverlay} />
    </div>
  </div>
  {#if $dev}
    {#each limbErrors as limb}
      <div>
        {limb.name.split(' ')[0]}
      </div>
    {/each}
  {/if}
</div>

<!-- <Banner
  steps={detectedSteps}
  bind:setCursor
  reviewStart={recordingStart}
  reviewEnd={recordingEnd}
  onScroll={seekVideoToCursor}
></Banner> -->

{#if $dev}
  <AllPoseErrors {reviewVideoElement} {recordingStart}></AllPoseErrors>
{/if}

<style>
  video {
    border-radius: 10px;
    overflow: hidden;
  }

  .video-wrapper {
    position: relative;
    margin: auto;
  }

  .video-overlay {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
  }

  .upper {
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
  }

  .poses-details {
    display: flex;
    overflow-x: auto;
    max-width: 100%;
  }

  .background-strip {
    margin: 10px -25px;
    padding: 10px 30px;
    border-radius: 10px;
    box-shadow: 0px 0px 8px rgba(0, 0, 0, 0.55);
    text-align: center;
  }

  .beat-label {
    margin-bottom: 10px;
  }

  .toggle-item {
    display: flex;
    justify-content: space-around;
    align-items: center;
  }
</style>
