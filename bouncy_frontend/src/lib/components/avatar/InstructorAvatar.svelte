<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '../avatar/Svg.svelte';
  import { onMount } from 'svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  export let nextSkeleton;
  export let nextBodyShift;

  const avatarLineWidth = 5;
  const correctAvatarLineWidth = 10;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let prevSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let prevBodyShift = null;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let correctSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let correctBodyShift = null;
  let showCorrectPosition = false;

  $: if (nextSkeleton !== prevSkeleton) {
    correctSkeleton = prevSkeleton;
    correctBodyShift = prevBodyShift;
    prevSkeleton = nextSkeleton;
    prevBodyShift = nextBodyShift;
    displayCorrectPosition();
  }

  function displayCorrectPosition() {
    showCorrectPosition = true;
    setTimeout(() => {
      // TODO: handle reentrance
      showCorrectPosition = false;
    }, 500);
  }

  onMount(() => {
    prevSkeleton = nextSkeleton;
    prevBodyShift = nextBodyShift;
  });
</script>

<div class="avatar-container back">
  {#if !showCorrectPosition}
    <Svg {width} {height}>
      <SvgAvatar
        skeleton={nextSkeleton}
        {width}
        {height}
        leftColor={'#e97516C0'}
        rightColor={'#e97516C0'}
        headColor={'#ffad6960'}
        bodyColor={'#ffad6940'}
        lineWidth={avatarLineWidth}
        bodyShift={nextBodyShift}
      ></SvgAvatar>
    </Svg>
  {/if}
</div>

<div class="avatar-container front">
  {#if showCorrectPosition && correctSkeleton}
    <Svg {width} {height}>
      <SvgAvatar
        {width}
        {height}
        skeleton={correctSkeleton}
        leftColor={'#4caf50'}
        rightColor={'#4caf50'}
        headColor={'#8bc34a'}
        bodyColor={'#c8e6c9'}
        lineWidth={correctAvatarLineWidth}
        bodyShift={correctBodyShift}
      ></SvgAvatar>
    </Svg>
  {/if}
</div>

<style>
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
  }
  .front {
    z-index: 2;
  }
  .back {
    z-index: 1;
  }
</style>
