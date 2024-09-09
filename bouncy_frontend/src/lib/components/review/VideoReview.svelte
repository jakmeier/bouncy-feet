<script>
  import { dev } from '$lib/stores/FeatureSelection.js';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { getContext, onMount } from 'svelte';
  import BackgroundTask from '$lib/components/BackgroundTask.svelte';
  import PoseReview from './PoseReview.svelte';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';

  /** @type {string} URL (usually local) to the video for review  */
  export let reviewVideoSrc;
  /** @type {number} */
  export let recordingStart;
  /** @type {number} */
  export let recordingEnd;
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let detectedSteps = [];

  $: firstPoseTime = detectedSteps[0].start;
  $: lastPoseTime = detectedSteps[detectedSteps.length - 1].end;

  let tracker = getContext('tracker').tracker;
  const avatarSize = 140;
  const avatarLineWidth = 5;

  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  let skeleton;

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

  function togglePlay() {
    if (reviewVideoElement.paused) {
      reviewVideoElement.play();
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
  }

  onMount(() => {
    selectTimestamp(firstPoseTime);
  });
</script>

<!-- update banner position on every frame, to keep it in sync with the video
even when it is playing. In theory, `on:timeupdate={onSeek}` in the video would
be better, however, this fires at different rates per browser and often only
once per 250ms. -->
<BackgroundTask {onFrame}></BackgroundTask>

<div class="poses-details" bind:this={poseCards} on:scroll={onScrollPoseCards}>
  {#each detectedSteps as step}
    {#each step.poses as pose}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div on:click={() => selectTimestamp(pose.timestamp)}>
        <PoseReview {pose}></PoseReview>
      </div>
    {/each}
  {/each}
</div>

<div class="background-strip">
  <div class="upper">
    <div>
      <!-- svelte-ignore a11y-media-has-caption -->
      <video
        bind:this={reviewVideoElement}
        on:click={togglePlay}
        src={reviewVideoSrc}
        playsinline
        style="margin-top: 10px; max-width: 100%"
      ></video>
    </div>

    <div>
      <Svg height={avatarSize} width={avatarSize}>
        {#if skeleton}
          <SvgAvatar
            width={avatarSize}
            height={avatarSize}
            lineWidth={avatarLineWidth}
            {skeleton}
            style={LEFT_RIGHT_COLORING}
          />
        {/if}
      </Svg>
    </div>
  </div>
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

  .upper {
    display: grid;
    grid-template-columns: 2fr 1fr;
    align-items: center;
  }

  .poses-details {
    display: flex;
    overflow-x: auto;
  }

  .background-strip {
    margin: 0 -25px;
    padding: 10px 30px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
  }
</style>
