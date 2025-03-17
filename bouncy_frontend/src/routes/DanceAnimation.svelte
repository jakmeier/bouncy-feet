<script>
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { MAIN_THEME_COLORING } from '$lib/constants';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { counter } from '$lib/timer';

  const bpm = 260;
  const stepTime = 60_000 / bpm;

  /**
   * @typedef {Object} Props
   * @property {any} dance
   * @property {number} [beatDelay] - How long it takes before the dance animation begins
   * @property {number} [hiddenBeats] - How many beats the avatar should be hidden at the start
   * @property {any} [beat]
   * @property {any} [animationTime]
   * @property {number} [size]
   * @property {boolean} [showOverflow]
   */

  /** @type {Props} */
  let {
    dance,
    beatDelay = 5,
    hiddenBeats = 0,
    beat = counter(-beatDelay, 1, stepTime),
    animationTime = stepTime * 0.85,
    showOverflow = false,
  } = $props();

  const size = 250;
  // When the beat is negative, it should show a resting position
  // according to the orientation of the first pose.
  const firstPost = dance.skeleton(0);
  const restingStep = Skeleton.resting(firstPost ? firstPost.sideway : false);

  /**
   * @param {import("bouncy_instructor").DanceWrapper} dance
   * @param {number} poseIndex
   */
  function danceBodyShift(dance, poseIndex) {
    const cartesian = dance.bodyShift(poseIndex);
    return { x: cartesian.x, y: cartesian.y };
  }
  let skeleton = $derived($beat >= 0 ? dance.skeleton($beat) : restingStep);
  let bodyShift = $derived(
    $beat >= 0 ? danceBodyShift(dance, $beat) : { x: 0, y: 0 }
  );
</script>

<Animation {animationTime} jumpHeight={size * 0.025}>
  <Svg height={size} width={size} orderByZ {showOverflow}>
    {#if skeleton && $beat >= hiddenBeats - beatDelay}
      <SvgAvatar width={size} height={size} {skeleton} {bodyShift} />
    {/if}
  </Svg>
</Animation>
