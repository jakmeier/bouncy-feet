<script>
  import SvgAvatar from '$lib/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { counter } from '$lib/timer';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceInfo} */
  export let dance;
  export let beat = counter(-6, 1, 200);
  export let animationTime = 160;
  export let size = 100;

  // When the beat is negative, it should show a resting position
  // according to the orientation of the first pose.
  const restingStep = Skeleton.resting(dance.skeleton(0).sideway);
  $: skeleton = $beat >= 0 ? dance.skeleton($beat) : restingStep;
</script>

<svg viewBox="0 0 {size} {size}">
  <SvgAvatar
    width={size}
    height={size}
    {skeleton}
    lineWidth={4}
    {animationTime}
  />
</svg>
