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
  let poseName = '';

  /** @param {PoseWrapper} newPose */
  export function loadPose(newPose) {
    // using an indirect setting, rather than a property, to better control when
    // the updates happen
    pose = newPose;
    // TODO: support translated editing
    let maybeName = pose.name('en');
    if (maybeName) {
      poseName = maybeName;
    } else {
      poseName = pose.id();
    }
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

  function updateName() {
    if (pose) {
      // TODO: support translated editing
      pose.setName(poseName, 'en');
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
  <div class="name">
    <input
      name="pose-name"
      class="name-input"
      bind:value={poseName}
      on:input={() => updateName()}
      placeholder="Name"
    />
  </div>

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
          on:change={() => updateAvatar(index)}
          placeholder="Weight"
          min="0.0"
        />
      </div>
    {/each}
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-areas:
      'avatar weights'
      'name weights';
  }
  .name {
    grid-area: name;
  }
  .avatar {
    grid-area: avatar;
    border: solid 1px var(--theme-neutral-gray);
    border-radius: 100px;
  }
  .weights {
    grid-area: weights;
    display: flex;
    flex-direction: column;
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
    font-size: 20px;
  }
  .name-input {
    border: none;
    width: 100%;
    text-align: center;
    font-size: 32px;
  }
</style>
