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
  import { onMount, untrack } from 'svelte';
  import AvatarStyleContext from './AvatarStyleContext.svelte';

  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").Skeleton} skeleton
   * @property {Cartesian2d} [bodyShift]
   * @property {boolean} [lastPoseWasCorrect]
   * @property {Cartesian2d} [origin]
   * @property {number} [avatarSize]
   * @property {AvatarColoring} [instructorStyle]
   * @property {number} [showCorrectTime]
   * @property {number} [animationTime]
   * @property {number} [jumpHeight]
   */

  /** @type {Props} */
  let {
    skeleton,
    bodyShift = new Cartesian2d(0, 0),
    lastPoseWasCorrect = true,
    origin = new Cartesian2d(0.0, 0.0),
    avatarSize = 1.0,
    instructorStyle = {
      leftColor: '#000000FF',
      rightColor: '#000000FF',
      headColor: '#00000040',
    },
    showCorrectTime = 100,
    animationTime = 100,
    jumpHeight = 1.0,
  } = $props();

  // This would work to show one pose ahead of time.
  // But currently, the pose is switched right when the animation should start,
  // so the delay can be 0.
  // let animationDelay = $timeBetweenMoves - animationTime;
  let animationDelay = $state(0);

  /** @type {import("bouncy_instructor").Skeleton | null} */
  let prevSkeleton = $state(null);
  /** @type {import('bouncy_instructor').Cartesian2d | null} */
  let prevBodyShift = $state(null);

  /** @type {import("bouncy_instructor").Skeleton | null} */
  let correctSkeleton = $state(null);
  /** @type {import('bouncy_instructor').Cartesian2d | null} */
  let correctBodyShift = $state(null);

  let coloring = $state(instructorStyle);
  let displayedSkeleton = $state(skeleton);
  let displayedBodyShift = $state(bodyShift);

  // This became the standard size for all SVGs, making for a standard width of
  // lines in SVG. Here it's also used for jump height normalization.
  const svgSize = 250;

  $effect(() => {
    if (skeleton !== prevSkeleton) {
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
  });

  function displayCorrectPosition() {
    coloring = CORRECT_COLORING;
    displayedBodyShift = correctBodyShift || bodyShift;
    setTimeout(() => {
      // TODO: handle reentrance
      animationDelay = $timeBetweenMoves - animationTime - showCorrectTime;
      coloring = instructorStyle;
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
  <Animation
    {animationTime}
    delay={animationDelay}
    jumpHeight={jumpHeight * svgSize}
  >
    <Svg width={svgSize} height={svgSize} orderByZ showOverflow>
      <AvatarStyleContext {coloring}>
        <SvgAvatar
          skeleton={displayedSkeleton}
          width={svgSize}
          height={svgSize}
          {avatarSize}
          bodyShift={displayedBodyShift.add(origin)}
        ></SvgAvatar>
      </AvatarStyleContext>
    </Svg>
  </Animation>
</div>

<style>
  .avatar-container {
    width: 100%;
    height: 100%;
  }
</style>
