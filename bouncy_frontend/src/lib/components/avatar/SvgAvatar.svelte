<script>
  import { Cartesian2d, LimbError } from '$lib/instructor/bouncy_instructor';
  import { MAIN_THEME_COLORING } from '$lib/constants';
  import SvgAvatar2 from './SvgAvatar2.svelte';

  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let bodyShift = { x: 0, y: 0 };
  export let avatarSize = 1.0;
  /** @type LimbError[] */
  export let markedLimbs = [];
  /** @type number[] */
  export let markedLimbIndices = [];

  /** @type {AvatarColoring} */
  export let style = MAIN_THEME_COLORING;

  /**
   * @param {number} s
   * @param {number} min
   * @param {number} max
   */
  function wrap(s, min, max) {
    let wrapped = (s - min) % (max - min);
    if (wrapped < 0) {
      wrapped += max - min;
    }
    return wrapped + min;
  }

  $: avatarSizePixels = Math.min(height, width) * avatarSize;
  $: hip = {
    x: (0.5 + wrap(bodyShift.x, -0.75, 0.75)) * width,
    y: (0.5 + bodyShift.y) * height,
  };

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonV2} */
  $: renderedSkeleton = skeleton.render(
    new Cartesian2d(hip.x, hip.y),
    avatarSizePixels
  );
  let dummyUpdate = 0;
  $: renderedSkeleton, (dummyUpdate += 1);
  $: markedSegments = markedLimbs
    .map((limb) => limb.render(renderedSkeleton))
    .concat(markedLimbIndices.map((i) => renderedSkeleton.segment(i)));

  $: headRadius = 0.075 * avatarSizePixels;
</script>

<SvgAvatar2
  {avatarSizePixels}
  skeleton={renderedSkeleton}
  {lineWidth}
  {markedSegments}
  {headRadius}
  {style}
/>
