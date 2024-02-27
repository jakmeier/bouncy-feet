<script>
  import Area from '$lib/components/Area.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
  export let rotation = 0.0;
  export let size = 100;
  export let poseIndex = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;

  // When the pose index is negative, it should show a resting position
  // according to the orientation of the first pose.
  const restingStep = Skeleton.resting(step.skeleton(0).sideway);
  $: skeleton = poseIndex >= 0 ? step.rotatedSkeleton(poseIndex, rotation) : restingStep;
</script>

<Area width="{size}px" height="{size}px">
  <svg viewBox="0 0 {size} {size}">
    <SvgAvatar
      width={size}
      height={size}
      {skeleton}
      lineWidth={4}
      leftColor="var(--theme-accent)"
      rightColor="var(--theme-main)"
      headColor="var(--theme-neutral-dark)"
      {animationTime}
    />
  </svg>
</Area>
