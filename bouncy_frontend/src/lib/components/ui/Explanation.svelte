<script>
  import { run } from 'svelte/legacy';

  import Animation from '$lib/components/avatar/Animation.svelte';
  import SpeechBubble from '$lib/components/ui/SpeechBubble.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import {
    Cartesian2d,
    Skeleton,
    DanceWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import { onMount } from 'svelte';
  import { counter } from '$lib/timer';

  /**
   * @typedef {Object} Props
   * @property {any} text
   * @property {number} [width]
   * @property { null | DanceWrapper } [entryDance]
   * @property {number} [entryDanceRepetitions]
   */

  /** @type {Props} */
  let {
    text,
    width = 250,
    entryDance = null,
    entryDanceRepetitions = 1,
  } = $props();
  /** @type { undefined | Skeleton } */
  let skeleton = $state(Skeleton.resting(false));
  let bodyShift = $state(new Cartesian2d(0, 0));
  const beat = counter(-1, 1, 227);

  const tailSize = 18;

  /** @param {number} width */
  function selectLineWidth(width) {
    if (width <= 150) {
      return 2;
    }
    if (width <= 250) {
      return 5;
    }
    if (width <= 350) {
      return 8;
    }
    return 10;
  }

  let firstDancedBeat = Infinity;
  const dancedBeats = (entryDance?.subbeats || 0) * entryDanceRepetitions;

  function updateSkeleton() {
    if (entryDance) {
      const n = $beat - firstDancedBeat;
      if (n >= 0 && n < dancedBeats) {
        skeleton = entryDance.skeleton(n);
        bodyShift = entryDance.bodyShift(n);
      } else if (n === dancedBeats) {
        skeleton = Skeleton.resting(false);
        bodyShift = entryDance.bodyShift(n);
      }
    }
  }

  function restartAnimation() {
    const n = $beat - firstDancedBeat;
    if (n > dancedBeats) {
      firstDancedBeat = $beat + 2;
    }
  }

  onMount(() => {
    firstDancedBeat = $beat + 2;
  });
  let figureWidth = $derived((width * 2) / 3);
  let bubbleTailRightOffset = $derived(`${figureWidth / 2 - 2 * tailSize}px`);
  let lineWidth = $derived(selectLineWidth(width));
  run(() => {
    $beat, updateSkeleton();
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="explanation" style="width: {width}px" onclick={restartAnimation}>
  <SpeechBubble
    {text}
    position="bottom"
    right={bubbleTailRightOffset}
    width={'100%'}
    {tailSize}
  />

  <div class="figure-cell" style="height: {figureWidth + tailSize}px">
    <div class="figure" style="max-width: {figureWidth}px;">
      <Animation animationTime={200} jumpHeight={0.025}>
        <Svg width={250} height={250} showOverflow>
          {#if skeleton}
            <SvgAvatar {skeleton} width={250} height={250} {bodyShift}
            ></SvgAvatar>
          {/if}
        </Svg>
      </Animation>
    </div>
  </div>
</div>

<style>
  .figure-cell {
    /* for positioning the .figure */
    position: relative;
  }
  .figure {
    position: absolute;
    right: 0;
    margin-top: 7px;
  }
  .explanation {
    margin: auto;
  }
</style>
