<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';

  /** @type {import('$lib/instructor/bouncy_instructor').StepWrapper} */
  export let step;
  export let rotation = 0.0;
  export let size = 100;
  export let poseIndex = 0;
  export let borderWidth = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;
  export let lineWidth = size * 0.05;
  export let style = LEFT_RIGHT_COLORING;
  export let backgroundColor = 'var(--theme-neutral-light)';

  export let headRadius = undefined;
  export let headHeight = undefined;

  // When the pose index is negative, it should show a resting position
  // according to the orientation of the first pose.
  const restingStep = Skeleton.resting(step.skeleton(0).sideway);
  // TODO: rotation and flipped orientation does not work
  // $: skeleton =
  //   poseIndex >= 0 ? step.rotatedSkeleton(poseIndex, rotation) : restingStep;
  $: skeleton =
    poseIndex >= 0
      ? rotation === 0
        ? step.skeleton(poseIndex)
        : step.rotatedSkeleton(poseIndex, rotation)
      : restingStep;
  $: bodyShift =
    poseIndex >= 0 ? stepBodyShift(step, poseIndex) : { x: 0, y: 0 };
  $: maybeJumpHeight = step.jumpHeight(poseIndex);
  $: jumpHeight =
    size * (maybeJumpHeight === undefined ? 1.0 : maybeJumpHeight);

  /**
   * @param {import("$lib/instructor/bouncy_instructor").StepWrapper} step
   * @param {number} poseIndex
   */
  function stepBodyShift(step, poseIndex) {
    const cartesian = step.bodyShift(poseIndex);
    return { x: cartesian.x, y: cartesian.y };
  }
</script>

<Area
  width="{size}px"
  height="{size}px"
  borderRadius="20px"
  borderWidth="{borderWidth}px"
  {backgroundColor}
>
  <Animation {animationTime} {jumpHeight}>
    <Svg width={size} height={size} orderByZ>
      <SvgAvatar
        width={size}
        height={size}
        {skeleton}
        {bodyShift}
        {lineWidth}
        {style}
        {headRadius}
        {headHeight}
      />
    </Svg>
  </Animation>
</Area>
