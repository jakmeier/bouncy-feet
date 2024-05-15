<script>
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { counter } from '$lib/timer';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceInfo} */
  export let dance;
  const bpm = 260;
  const stepTime = 60_000 / bpm;
  export let beat = counter(-5, 1, stepTime);
  export let animationTime = stepTime * 0.85;
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
  $: bodyShift = $beat >= 0 ? danceBodyShift(dance, $beat) : { x: 0, y: 0 };

  /**
   * @param {import("$lib/instructor/bouncy_instructor").DanceInfo} dance
   * @param {number} poseIndex
   */
  function danceBodyShift(dance, poseIndex) {
    const cartesian = dance.bodyShift(poseIndex);
    return { x: cartesian.x, y: cartesian.y };
  }
</script>

<Animation {animationTime}>
  <Svg height={size} width={size} orderByZ>
    {#if skeleton}
      <SvgAvatar
        width={size}
        height={size}
        {skeleton}
        {bodyShift}
        lineWidth={4}
        {leftColor}
        {rightColor}
        {headColor}
        {bodyColor}
      />
    {/if}
  </Svg>
</Animation>
