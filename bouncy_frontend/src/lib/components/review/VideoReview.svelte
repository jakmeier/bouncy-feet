<script>
  import { run } from 'svelte/legacy';

  import { dev } from '$lib/stores/FeatureSelection.js';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import { getContext, onMount } from 'svelte';
  import BackgroundTask from '$lib/components/BackgroundTask.svelte';
  import PoseReview from './PoseReview.svelte';
  import {
    DanceCursor,
    LimbError,
    PoseApproximation,
  } from '$lib/instructor/bouncy_instructor';
  import SvgAvatar2 from '../avatar/SvgAvatar2.svelte';
  import Toggle from '../ui/Toggle.svelte';
  import { t } from '$lib/i18n';
  import LightSection from '../ui/sections/LightSection.svelte';
  import { selectMediaMimeType } from '$lib/media';
  import CornerMarker from '../ui/CornerMarker.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} reviewVideoSrc
   * @property {number} recordingStart
   * @property {number} recordingEnd
   * @property {import("bouncy_instructor").DetectedStep[]} [detectedSteps]
   */

  /** @type {Props} */
  let {
    reviewVideoSrc,
    recordingStart,
    recordingEnd,
    detectedSteps = [],
  } = $props();

  let videoSrcWidth = $state(0);
  let videoSrcHeight = $state(0);
  let videoLoaded = $state(false);
  let displayVideoOverlay = $state(true);

  let firstPoseTime = $derived(
    detectedSteps.length > 0 ? detectedSteps[0].start : 0
  );
  let lastPoseTime = $derived(
    detectedSteps.length > 0 ? detectedSteps[detectedSteps.length - 1].end : 100
  );

  let tracker = getContext('tracker').tracker;

  /** @type {HTMLVideoElement} */
  let reviewVideoElement = $state();
  /** @type {import("bouncy_instructor").Skeleton} */
  let skeleton;
  /** @type {import("bouncy_instructor").SkeletonV2} */
  let keypointSkeleton = $state();
  /** @type {LimbError[]} */
  let limbErrors = $state([]);
  let danceCursor = $state(new DanceCursor());

  let avatarSizePixels = $derived(videoSrcHeight);
  /** @type {import("bouncy_instructor").RenderableSegment[]} */
  let markedSegments = $state([]);
  run(() => {
    if (keypointSkeleton) {
      markedSegments = limbErrors.map((limb) => limb.render(keypointSkeleton));
    }
  });

  let cameraNotSupported = $derived(
    (reviewVideoSrc === '' ||
      !videoLoaded ||
      reviewVideoElement?.duration === 0) &&
      selectMediaMimeType() === null
  );

  const numSubbeats = $derived(tracker.trackedSubbeats);

  let prevTime = 0;
  function onFrame() {
    if (reviewVideoElement && prevTime !== reviewVideoElement.currentTime) {
      onSeek();
      prevTime = reviewVideoElement.currentTime;
    }
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
  }

  // Set the position in the video and the banner to the specified recording timestamp.
  /** @param {number} timestamp */
  function selectTimestamp(timestamp) {
    const videoTime = (Number(timestamp) - recordingStart) / 1000;
    reviewVideoElement.currentTime = videoTime;
  }

  /** @returns {null | PoseApproximation}*/
  function selectedPose() {
    const pose = getPoseApproximation(danceCursor);
    if (!pose) {
      console.warn(
        'beat out of range',
        danceCursor.subbeat,
        danceCursor.stepIndex,
        danceCursor.poseIndex
      );
      return null;
    }
    return pose;
  }

  // Load the specified beat to the review cursor.
  /**
   * @param {DanceCursor} newDanceCursor
   * */
  function selectPose(newDanceCursor) {
    danceCursor = newDanceCursor;
    const pose = selectedPose();
    if (!pose) {
      return;
    }
    selectTimestamp(pose.timestamp);
    // TODO: deduplicate threshold definition
    const threshold = 0.075;
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
    danceCursor = tracker.cursor(t);
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
  let poseCards = $state();
  function onScrollPoseCards() {
    const r =
      poseCards.scrollLeft / (poseCards.scrollWidth - poseCards.clientWidth);
    const t = firstPoseTime + (lastPoseTime - firstPoseTime) * r;
    selectTimestamp(t);
    checkTimeOnBeat(t);
  }

  /**
   * @param {number} subbeat
   */
  function formatBeatLabel(subbeat) {
    if (subbeat % 2 === 1) {
      return '+';
    }
    return String(subbeat / 2 + 1);
  }

  /**
   * @param {DanceCursor} danceCursor
   * @returns {PoseApproximation | undefined}
   */
  function getPoseApproximation(danceCursor) {
    if (danceCursor.stepIndex > detectedSteps.length) {
      return;
    }
    const step = detectedSteps[danceCursor.stepIndex];
    const poses = step ? step.poses : [];
    if (poses.length === 0) {
      return;
    }
    return poses[danceCursor.poseIndex % poses.length];
  }

  onMount(() => {
    selectPose(danceCursor);
  });
</script>

<!-- update banner position on every frame, to keep it in sync with the video
even when it is playing. In theory, `on:timeupdate={onSeek}` in the video would
be better, however, this fires at different rates per browser and often only
once per 250ms. -->
<BackgroundTask {onFrame}></BackgroundTask>

<LightSection>
  <h2>{$t('record.review-title')}</h2>

  <div class="upper">
    <CornerMarker>
      <div class="video-wrapper">
        {#if cameraNotSupported}
          <p class="error">
            {$t('record.camera-not-supported')}
          </p>
        {:else if reviewVideoSrc === ''}
          <p class="error">
            {$t('record.no-video')}
          </p>
        {/if}

        <!-- svelte-ignore a11y_media_has_caption -->
        <video
          onclick={togglePlay}
          onloadeddata={() => {
            const isPlayable =
              reviewVideoElement.readyState >=
                reviewVideoElement.HAVE_FUTURE_DATA &&
              !reviewVideoElement.error;
            if (isPlayable) {
              videoLoaded = true;
              onSeek();
            }
          }}
          bind:this={reviewVideoElement}
          bind:videoWidth={videoSrcWidth}
          bind:videoHeight={videoSrcHeight}
          src={reviewVideoSrc}
          playsinline
          webkit-playsinline
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
                {markedSegments}
              />
            </Svg>
          </div>
        {/if}
      </div>
    </CornerMarker>

    <div class="toggle-item">
      <Toggle bind:isOn={displayVideoOverlay} />
      <div>{$t('record.settings.enable-tracking')}</div>
    </div>
  </div>

  <div class="poses-details" bind:this={poseCards} onscroll={onScrollPoseCards}>
    {#each { length: numSubbeats } as _, subbeat}
      {@const cursor = tracker.cursorAtSubbeat(subbeat)}
      {@const prevCursor =
        subbeat > 0 ? tracker.cursorAtSubbeat(subbeat - 1) : null}
      {@const isNewPose = !prevCursor || !cursor.isSamePose(prevCursor)}
      {#if isNewPose}
        {@const pose = getPoseApproximation(cursor)}
        {#if pose}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div onclick={() => selectPose(cursor)}>
            <PoseReview
              {pose}
              beatLabel={formatBeatLabel(subbeat)}
              skeleton={tracker.poseSkeletonAt(cursor)}
            ></PoseReview>
          </div>
        {:else}
          <div class="missing-pose-placeholder">
            <div>·</div>
          </div>
        {/if}
      {:else if subbeat % 2 === 0}
        <div class="same-pose-placeholder">
          <div>·</div>
        </div>
      {/if}
    {/each}
  </div>

  {#if $dev}
    <AllPoseErrors {reviewVideoElement} {recordingStart}></AllPoseErrors>
    <div>expected pose: {selectedPose()?.id}</div>
  {/if}
</LightSection>

<style>
  video {
    /* border-radius: 10px; */
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
    overflow: hidden;
  }

  .upper {
    display: grid;
    grid-template-columns: 1fr;
    align-items: center;
    width: fit-content;
    margin: auto;
  }

  .poses-details {
    display: flex;
    overflow-x: auto;
    /* Use space all the way to the edge */
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }

  .missing-pose-placeholder,
  .same-pose-placeholder {
    display: inline-block;
    margin: 1rem 1px;
  }
  .missing-pose-placeholder div,
  .same-pose-placeholder div {
    color: var(--theme-neutral-white);
    background-color: var(--theme-neutral-black);
    border-radius: 50%;
    height: 1rem;
    width: 1rem;
    /* padding: 0.125rem; */

    text-align: center;
    align-content: center;
    justify-content: center;
    font-size: 1rem;
    vertical-align: middle;
  }
  .missing-pose-placeholder div {
    background-color: var(--theme-accent);
  }

  .toggle-item {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-wrap: wrap;
    margin: 1rem;
    gap: 1rem;
  }

  .error {
    /* background-color: var(--theme-neutral-black); */
    padding: 1rem;
    color: var(--theme-accent);
  }
</style>
