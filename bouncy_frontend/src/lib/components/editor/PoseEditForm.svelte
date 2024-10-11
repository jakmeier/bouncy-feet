<script>
  import { GRAY_COLORING } from '$lib/constants';
  import {
    PoseWrapper,
    SkeletonField,
  } from '$lib/instructor/bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import SvgAvatar from '../avatar/SvgAvatar.svelte';

  /** @type {PoseWrapper} */
  export let pose;

  $: skeleton = pose.skeleton();
  $: weightedLimbs = extractWeightedLimbs(pose);
  $: markedLimbIndices = weightedLimbs.map((limb) => limb.index);

  /**
   * @param {PoseWrapper} pose
   * @returns {WeightedPoseLimb[]}
   */
  function extractWeightedLimbs(pose) {
    // using reduce as a map + filter to avoid the extra array
    return Object.keys(SkeletonField).reduce(
      /** @type {(acc: WeightedPoseLimb[], field: string) => WeightedPoseLimb[]} */
      (acc, field) => {
        // enum has static values for indices and names, we want to filter only the numbers
        let limb = SkeletonField[field];
        if (typeof limb === 'number') {
          let weight = pose.getWeight(limb);
          if (weight > 0.0) {
            acc.push({ name: SkeletonField[limb], index: limb, weight });
          }
        }
        return acc;
      },
      /** @type {WeightedPoseLimb[]}*/
      []
    );
  }
</script>

<div class="avatar">
  <Svg width={300} height={300} orderByZ>
    {#if skeleton}
      <SvgAvatar
        {skeleton}
        width={300}
        height={300}
        style={GRAY_COLORING}
        {markedLimbIndices}
      ></SvgAvatar>
    {/if}
  </Svg>
</div>
