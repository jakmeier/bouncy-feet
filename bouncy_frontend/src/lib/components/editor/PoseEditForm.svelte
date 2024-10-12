<script>
  import { GRAY_COLORING } from '$lib/constants';
  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    Skeleton,
    SkeletonField,
  } from '$lib/instructor/bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import SvgAvatar from '../avatar/SvgAvatar.svelte';

  /** @type {PoseWrapper | undefined} */
  let pose;
  /** @type {Skeleton | undefined} */
  let skeleton;
  /** @type {WeightedPoseLimb[]} */
  let weightedLimbs = [];

  /** @param {PoseWrapper} newPose */
  export function loadPose(newPose) {
    // using an indirect setting, rather than a property, to better control when
    // the updates happen
    pose = newPose;
    onPoseUpdated(pose);
  }

  /** @param {PoseWrapper} newPose */
  function onPoseUpdated(newPose) {
    skeleton = newPose.skeleton();
    weightedLimbs = extractWeightedLimbs(newPose);
  }

  $: markedLimbIndices = weightedLimbs
    .filter((limb) => limb.weight > 0.0)
    .map((limb) => limb.index);

  /**
   * @param {PoseWrapper} pose
   * @returns {WeightedPoseLimb[]}
   */
  function extractWeightedLimbs(pose) {
    return allSkeletonFields().map((limb) => ({
      name: SkeletonField[limb],
      index: limb,
      weight: pose.getWeight(limb),
    }));
  }

  /** @param {number} weightedLimbIndex */
  function updateAvatar(weightedLimbIndex) {
    let limb = weightedLimbs[weightedLimbIndex];
    if (pose && limb) {
      pose.setWeight(limb.index, limb.weight);
      onPoseUpdated(pose);
    } else {
      console.log('Pose or limb not available');
    }
  }

  function allSkeletonFields() {
    // using reduce as a map + filter to avoid the extra array
    return Object.keys(SkeletonField).reduce(
      /** @type {(acc: number[], field: string) => number[]} */
      (acc, field) => {
        // enum has static values for indices and names, we want to filter only the numbers
        let limb = SkeletonField[field];
        if (typeof limb === 'number') {
          acc.push(limb);
        }
        return acc;
      },
      /** @type {number[]}*/
      []
    );
  }
</script>

<div class="container">
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

  <div class="weights">
    {#each weightedLimbs as limb, index}
      <div class="body-part">
        <label for={limb.name}>{$t(limb.name)}</label>
        <input
          name={limb.name}
          type="number"
          class="weight-input"
          bind:value={limb.weight}
          on:input={() => updateAvatar(index)}
          placeholder="Weight"
        />
      </div>
    {/each}
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 2fr 1fr;
  }
  .body-part {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 90%;
    margin: auto;
  }
  .weight-input {
    width: 100%;
    text-align: center;
  }
</style>
