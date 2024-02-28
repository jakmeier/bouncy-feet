<script>
  import { dev } from '$app/environment';
  import AllPoseErrors from '$lib/components/dev/AllPoseErrors.svelte';
  import Area from '$lib/components/Area.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Banner from './Banner.svelte';
  import { getContext } from 'svelte';
  import BackgroundTask from '$lib/components/BackgroundTask.svelte';

  /** @type {string} URL (usually local) to the video for review  */
  export let reviewVideoSrc;
  /** @type {number} */
  export let recordingStart;
  /** @type {number} */
  export let recordingEnd;
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let detectedSteps = [];

  let tracker = getContext('tracker').tracker;

  /** @type {HTMLVideoElement} */
  let reviewVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  let skeleton;

  async function onSeek() {
    const ms = reviewVideoElement.currentTime * 1000;
    const reviewTimestamp = ms + recordingStart;
    skeleton = tracker.skeletonAt(reviewTimestamp);
    const cursor =
      (reviewTimestamp - recordingStart) / (recordingEnd - recordingStart);
    setCursor(cursor);
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
</script>

<!-- update banner position on every frame, to keep it in sync with the video
even when it is playing. In theory, `on:timeupdate={onSeek}` in the video would
be better, however, this fires at different rates per browser and often only
once per 250ms. -->
<BackgroundTask onFrame={onSeek}></BackgroundTask>

<!-- svelte-ignore a11y-media-has-caption -->
<video
  bind:this={reviewVideoElement}
  src={reviewVideoSrc}
  playsinline
  controls
  style="margin-top: 10px;"
></video>

<Area width="{280}px" height="{280}px">
  <svg viewBox="0 0 280 280">
    <SvgAvatar width={280} height={280} {skeleton} />
  </svg>
</Area>

<Banner
  steps={detectedSteps}
  bind:setCursor
  reviewStart={recordingStart || 0}
  reviewEnd={recordingEnd || 1}
  onScroll={seekVideoToCursor}
></Banner>

{#if dev}
  <AllPoseErrors {reviewVideoElement} {recordingStart}></AllPoseErrors>
{/if}
