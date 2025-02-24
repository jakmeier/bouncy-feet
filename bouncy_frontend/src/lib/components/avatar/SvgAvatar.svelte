<script>
  import { run } from 'svelte/legacy';

  import { Cartesian2d, LimbError } from '$lib/instructor/bouncy_instructor';
  import SvgAvatar2 from './SvgAvatar2.svelte';

  /**
   * @typedef {Object} Props
   * @property {any} skeleton
   * @property {number} [width]
   * @property {number} [height]
   * @property {any} [bodyShift]
   * @property {number} [avatarSize]
   * @property {any} [markedLimbs]
   * @property {any} [markedLimbIndices]
   */

  /** @type {Props} */
  let {
    skeleton,
    width = 100,
    height = 100,
    bodyShift = { x: 0, y: 0 },
    avatarSize = 1.0,
    markedLimbs = [],
    markedLimbIndices = [],
  } = $props();

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

  let avatarSizePixels = $derived(Math.min(height, width) * avatarSize);
  let hip = $derived({
    x: (0.5 + wrap(bodyShift.x, -0.75, 0.75)) * width,
    y: (0.5 + bodyShift.y) * height,
  });

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonV2} */
  let renderedSkeleton = $derived(
    skeleton.render(new Cartesian2d(hip.x, hip.y), avatarSizePixels)
  );
  let markedSegments = $derived(
    markedLimbs
      .map((limb) => limb.render(renderedSkeleton))
      .concat(markedLimbIndices.map((i) => renderedSkeleton.segment(i)))
  );
</script>

<SvgAvatar2 {avatarSizePixels} skeleton={renderedSkeleton} {markedSegments} />
