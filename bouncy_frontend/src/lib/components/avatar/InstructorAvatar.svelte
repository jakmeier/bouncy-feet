<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Cartesian2d, PoseHint } from '$lib/instructor/bouncy_instructor';
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
  /** @type {PoseHint} */
  export let hint = PoseHint.DontKnow;

  $: avatarLineWidth = 6 * avatarSize;
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

  const baseStyle = {
    leftColor: '#ffffffFF',
    rightColor: '#ffffffFF',
    headColor: '#ffffff40',
    bodyColor: '#ffffff10',
  };

  const leftRightColoring = {
    leftColor: '#e97516D0',
    rightColor: '#382eebD0',
    headColor: '#ffad6940',
    bodyColor: '#ffad6910',
  };

  /**
   * @param {PoseHint} inputHint
   */
  function selectStyle(inputHint) {
    switch (inputHint) {
      case PoseHint.LeftRight:
        return leftRightColoring;
      case PoseHint.ZOrder:
        return baseStyle;
      default:
        return baseStyle;
    }
  }
  $: instructorStyle = selectStyle(hint);

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
    <Svg {width} {height} orderByZ>
      <SvgAvatar
        {skeleton}
        {width}
        {height}
        {avatarSize}
        leftColor={instructorStyle.leftColor}
        rightColor={instructorStyle.rightColor}
        headColor={instructorStyle.headColor}
        bodyColor={instructorStyle.bodyColor}
        lineWidth={avatarLineWidth}
        bodyShift={bodyShift.add(origin)}
      ></SvgAvatar>
    </Svg>
  {/if}
</div>

<div class="avatar-container front">
  {#if showCorrectPosition && correctSkeleton && correctBodyShift}
    <Svg {width} {height} orderByZ>
      <SvgAvatar
        {width}
        {height}
        {avatarSize}
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
