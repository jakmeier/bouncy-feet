<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import Animation from '$lib/components/avatar/Animation.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { CORRECT_COLORING } from '$lib/constants';
  import { Cartesian2d } from '$lib/instructor/bouncy_instructor';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import Svg from '../avatar/Svg.svelte';
  import { onMount } from 'svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  export let skeleton;
  /** @type {Cartesian2d} */
  export let bodyShift = new Cartesian2d(0, 0);
  /** @type {boolean} */
  export let lastPoseWasCorrect = true;

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

  export let showCorrectTime = 100;
  export let animationTime = 100;

  // This would work to show one pose ahead of time.
  // But currently, the pose is switched right when the animation should start,
  // so the delay can be 0.
  // let animationDelay = $timeBetweenMoves - animationTime;
  let animationDelay = 0;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let prevSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let prevBodyShift = null;

  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | null} */
  let correctSkeleton = null;
  /** @type {import('$lib/instructor/bouncy_instructor').Cartesian2d | null} */
  let correctBodyShift = null;

  let displayedStyle = instructorStyle;
  let displayedSkeleton = skeleton;
  let displayedBodyShift = bodyShift;

  $: if (skeleton !== prevSkeleton) {
    correctSkeleton = prevSkeleton;
    correctBodyShift = prevBodyShift;
    prevSkeleton = skeleton;
    prevBodyShift = bodyShift;
    // TODO: showing the correct position messes with the timing
    // if (lastPoseWasCorrect) {
    //   displayCorrectPosition();
    // } else {
    //   // animationDelay = $timeBetweenMoves - animationTime;
    //   // animationDelay = 0;
    displayedBodyShift = bodyShift;
    displayedSkeleton = skeleton;
    // }
  }

  function displayCorrectPosition() {
    displayedStyle = CORRECT_COLORING;
    displayedBodyShift = correctBodyShift || bodyShift;
    setTimeout(() => {
      // TODO: handle reentrance
      animationDelay = $timeBetweenMoves - animationTime - showCorrectTime;
      displayedStyle = instructorStyle;
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
  <Animation {animationTime} delay={animationDelay} jumpHeight={height * 0.025}>
    <Svg {width} {height} orderByZ>
      <SvgAvatar
        skeleton={displayedSkeleton}
        {width}
        {height}
        {avatarSize}
        style={displayedStyle}
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
