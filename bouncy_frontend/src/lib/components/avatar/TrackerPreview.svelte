<script>
  // Show what the tracker instructor will show in the live activity, as an
  // animation.

  import Area from '$lib/components/ui/Area.svelte';
  import Animation from '$lib/components/avatar/Animation.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { timeBetweenMoves, beatCounter } from '$lib/stores/Beat';

  /**
   * @typedef {Object} Props
   * @property {Tracker} tracker
   * @property {number} [size]
   * @property {string} [backgroundColor]
   */

  /** @type {Props} */
  let {
    tracker,
    size = 100,
    backgroundColor = 'var(--theme-neutral-light)',
  } = $props();

  let skeleton = $derived(tracker.poseSkeletonAtSubbeat($beatCounter));
  let bodyShift = $derived(tracker.poseBodyShiftAtSubbeat($beatCounter));
  let animationTime = $derived($timeBetweenMoves * 0.8);
</script>

<Area
  width="{size}px"
  height="{size}px"
  borderWidth="0"
  borderRadius="20px"
  {backgroundColor}
>
  <Animation {animationTime}>
    <Svg width={250} height={250} orderByZ>
      <SvgAvatar width={250} height={250} {skeleton} {bodyShift} />
    </Svg>
  </Animation>
</Area>
