<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import Animation from '$lib/components/avatar/Animation.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { CORRECT_COLORING } from '$lib/constants';
  import { Cartesian2d } from '$lib/instructor/bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import { onMount } from 'svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  export let skeleton;
  /** @type {Cartesian2d} */
  export let bodyShift;

  /** @type {Cartesian2d} */
  export let origin = new Cartesian2d(0.0, 0.0);
  export let avatarSize = 1.0;

  /** @type {AvatarColoring} */
  export let instructorStyle = {
    leftColor: '#000000FF',
    rightColor: '#000000FF',
    headColor: '#00000040',
    bodyColor: '#00000010',
  };

  export let slowInstructor = false;
  $: avatarLineWidth = 6 * avatarSize;
  $: correctAvatarLineWidth = 10 * avatarSize;

  const showCorrectTime = slowInstructor ? 500 : 100;
  const animationTime = slowInstructor ? 400 : 200;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let prevSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let prevBodyShift = null;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let correctSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let correctBodyShift = null;

  let displayedStyle = instructorStyle;
  let displayedLineWidth = avatarLineWidth;
  let displayedSkeleton = skeleton;
  let displayedBodyShift = bodyShift;

  $: if (skeleton !== prevSkeleton) {
    correctSkeleton = prevSkeleton;
    correctBodyShift = prevBodyShift;
    prevSkeleton = skeleton;
    prevBodyShift = bodyShift;
    displayCorrectPosition();
  }

  function displayCorrectPosition() {
    displayedStyle = CORRECT_COLORING;
    displayedLineWidth = correctAvatarLineWidth;
    displayedBodyShift = correctBodyShift || bodyShift;
    setTimeout(() => {
      // TODO: handle reentrance
      displayedStyle = instructorStyle;
      displayedLineWidth = avatarLineWidth;
      displayedBodyShift = bodyShift;
      displayedSkeleton = skeleton;
    }, showCorrectTime);
  }

  onMount(() => {
    prevSkeleton = skeleton;
    prevBodyShift = bodyShift;
  });
</script>

<div class="avatar-container">
  <Animation {animationTime} jumpHeight={height * 0.05}>
    <Svg {width} {height} orderByZ>
      <SvgAvatar
        skeleton={displayedSkeleton}
        {width}
        {height}
        {avatarSize}
        style={displayedStyle}
        lineWidth={avatarLineWidth}
        bodyShift={displayedBodyShift.add(origin)}
      ></SvgAvatar>
    </Svg>
  </Animation>
</div>

<style>
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
  }
</style>
