<script>
  import Animation from '$lib/components/avatar/Animation.svelte';
  import SpeechBubble from '$lib/components/ui/SpeechBubble.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import {
    Cartesian2d,
    Skeleton,
    DanceInfo,
  } from '$lib/instructor/bouncy_instructor';
  import { onMount } from 'svelte';
  import { counter } from '$lib/timer';

  export let text;
  export let width = 250;
  /** @type { null | DanceInfo } */
  export let entryDance = null;
  export let entryDanceRepetitions = 1;
  /** @type { undefined | Skeleton } */
  let skeleton = Skeleton.resting(false);
  let bodyShift = new Cartesian2d(0, 0);
  const beat = counter(-1, 1, 227);

  const tailSize = 18;
  $: figureWidth = (width * 2) / 3;
  $: bubbleTailRightOffset = `${figureWidth / 2 - 2 * tailSize}px`;

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
  $: lineWidth = selectLineWidth(width);

  let firstDancedBeat = Infinity;
  const dancedBeats = (entryDance?.beats || 0) * entryDanceRepetitions;
  $: $beat, updateSkeleton();

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
  onMount(() => {
    firstDancedBeat = $beat + 2;
  });
</script>

<div class="explanation" style="width: {width}px">
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
        <Svg width={figureWidth} height={figureWidth} showOverflow>
          {#if skeleton}
            <SvgAvatar
              {skeleton}
              width={figureWidth}
              height={figureWidth}
              {lineWidth}
              {bodyShift}
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
