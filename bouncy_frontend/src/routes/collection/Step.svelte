<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';

  
  
  /**
   * @typedef {Object} Props
   * @property {import('bouncy_instructor').StepWrapper} step
   * @property {number} [rotation]
   * @property {number} [size]
   * @property {number} [poseIndex]
   * @property {number} [borderWidth]
   * @property {number} [animationTime]
   * @property {string} [backgroundColor]
   */

  /** @type {Props} */
  let {
    step,
    rotation = 0.0,
    size = 100,
    poseIndex = 0,
    borderWidth = 0,
    animationTime = 500,
    backgroundColor = 'var(--theme-neutral-light)'
  } = $props();

  // When the pose index is negative, it should show a resting position
  // according to the orientation of the first pose.
  const restingStep = Skeleton.resting(step.skeleton(0).sideway);

  /**
   * @param {import("bouncy_instructor").StepWrapper} step
   * @param {number} poseIndex
   */
  function stepBodyShift(step, poseIndex) {
    const cartesian = step.bodyShift(poseIndex);
    return { x: cartesian.x, y: cartesian.y };
  }
  // TODO: rotation and flipped orientation does not work
  // $: skeleton =
  //   poseIndex >= 0 ? step.rotatedSkeleton(poseIndex, rotation) : restingStep;
  let skeleton =
    $derived(poseIndex >= 0
      ? rotation === 0
        ? step.skeleton(poseIndex)
        : step.rotatedSkeleton(poseIndex, rotation)
      : restingStep);
  let bodyShift =
    $derived(poseIndex >= 0 ? stepBodyShift(step, poseIndex) : { x: 0, y: 0 });
  let maybeJumpHeight = $derived(step.jumpHeight(poseIndex));
  let jumpHeight =
    $derived(size * (maybeJumpHeight === undefined ? 1.0 : maybeJumpHeight));
</script>

<Area
  width="{size}px"
  height="{size}px"
  borderRadius="20px"
  borderWidth="{borderWidth}px"
  {backgroundColor}
>
  <Animation {animationTime} {jumpHeight}>
    <Svg width={250} height={250} orderByZ>
      <SvgAvatar width={250} height={250} {skeleton} {bodyShift} />
    </Svg>
  </Animation>
</Area>
