<script>
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { counter } from '$lib/timer';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceInfo} */
  export let dance;
  export let beat = counter(-6, 1, 200);
  export let animationTime = 160;
  export let size = 100;

  /** @type{undefined|string} */
  export let leftColor = undefined;
  /** @type{undefined|string} */
  export let rightColor = undefined;
  /** @type{undefined|string} */
  export let headColor = undefined;
  /** @type{undefined|string} */
  export let bodyColor = undefined;

  // When the beat is negative, it should show a resting position
  // according to the orientation of the first pose.
  const firstPost = dance.skeleton(0);
  const restingStep = Skeleton.resting(firstPost ? firstPost.sideway : false);
  $: skeleton = $beat >= 0 ? dance.skeleton($beat) : restingStep;
</script>

<Animation {animationTime}>
  <Svg height={size} width={size} orderByZ>
    {#if skeleton}
      <SvgAvatar
        width={size}
        height={size}
        {skeleton}
        lineWidth={4}
        {leftColor}
        {rightColor}
        {headColor}
        {bodyColor}
      />
    {/if}
  </Svg>
</Animation>
