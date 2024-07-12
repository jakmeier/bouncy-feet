<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Cartesian2d } from '$lib/instructor/bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import { onMount } from 'svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  export let skeleton;
  export let bodyShift;

  /** @type {Cartesian2d} */
  export let origin = new Cartesian2d(0.0, 0.0);
  export let avatarSize = 1.0;
  $: lengths = {
    thigh: 0.2 * avatarSize,
    shin: 0.2 * avatarSize,
    torso: 0.25 * avatarSize,
    arm: 0.1 * avatarSize,
    forearm: 0.15 * avatarSize,
    foot: 0.05 * avatarSize,
    shoulder: 0.1 * avatarSize,
    hip: 0.06 * avatarSize,
  };

  $: avatarLineWidth = 4 * avatarSize;
  $: correctAvatarLineWidth = 10 * avatarSize;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let prevSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let prevBodyShift = null;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let correctSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let correctBodyShift = null;
  let showCorrectPosition = false;

  $: if (skeleton !== prevSkeleton) {
    correctSkeleton = prevSkeleton;
    correctBodyShift = prevBodyShift;
    prevSkeleton = skeleton;
    prevBodyShift = bodyShift;
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
    prevSkeleton = skeleton;
    prevBodyShift = bodyShift;
  });
</script>

<div class="avatar-container back">
  {#if !showCorrectPosition}
    <Svg {width} {height}>
      <SvgAvatar
        {skeleton}
        {width}
        {height}
        {lengths}
        leftColor={'#e97516D0'}
        rightColor={'#382eebD0'}
        headColor={'#ffad6940'}
        bodyColor={'#ffad6910'}
        lineWidth={avatarLineWidth}
        bodyShift={bodyShift.add(origin)}
      ></SvgAvatar>
    </Svg>
  {/if}
</div>

<div class="avatar-container front">
  {#if showCorrectPosition && correctSkeleton && correctBodyShift}
    <Svg {width} {height}>
      <SvgAvatar
        {width}
        {height}
        {lengths}
        skeleton={correctSkeleton}
        leftColor={'#4caf50'}
        rightColor={'#4caf50'}
        headColor={'#8bc34a'}
        bodyColor={'#c8e6c9'}
        lineWidth={correctAvatarLineWidth}
        bodyShift={correctBodyShift.add(origin)}
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
