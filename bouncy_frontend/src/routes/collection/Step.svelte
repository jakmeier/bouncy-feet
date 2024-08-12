<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
  export let rotation = 0.0;
  export let size = 100;
  export let poseIndex = 0;
  export let borderWidth = 2;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;

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

  /**
   * @param {import("$lib/instructor/bouncy_instructor").StepInfo} step
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
>
  <Animation {animationTime} jumpHeight={size * 0.05}>
    <Svg width={size} height={size} orderByZ>
      <SvgAvatar
        width={size}
        height={size}
        {skeleton}
        {bodyShift}
        lineWidth={4}
        style={LEFT_RIGHT_COLORING}
      />
    </Svg>
  </Animation>
</Area>
