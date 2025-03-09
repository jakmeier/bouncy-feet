<script>
  import { GRAY_COLORING } from '$lib/constants';
  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    Skeleton,
    SkeletonLimb,
  } from 'bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import SvgAvatar from '../avatar/SvgAvatar.svelte';

  /** @type {PoseWrapper | undefined} */
  let pose;
  /** @type {Skeleton | undefined} */
  let skeleton = $state();
  /** @type {WeightedPoseLimb[]} */
  let weightedLimbs = $state([]);
  let poseName = $state('');

  let { onChange = () => {} } = $props();

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
      updateName();
    }
    onPoseUpdated(pose);
  }

  /** @returns {PoseWrapper|undefined} newPose */
  export function getPose() {
    return pose;
  }

  /** @param {PoseWrapper} newPose */
  function onPoseUpdated(newPose) {
    skeleton = newPose.skeleton();
    weightedLimbs = extractWeightedLimbs(newPose);
  }

  let markedLimbIndices = $derived(weightedLimbs
    .filter((limb) => limb.weight > 0.0)
    .map((limb) => limb.index));

  /**
   * @param {PoseWrapper} pose
   * @returns {WeightedPoseLimb[]}
   */
  function extractWeightedLimbs(pose) {
    return allSkeletonLimbs().map((limb) => ({
      name: SkeletonLimb[limb],
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
      onChange();
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

  function allSkeletonLimbs() {
    // simple, fast, and full control over order:
    // (requires manually updating when enum changes)
    return [
      SkeletonLimb.LeftArm,
      SkeletonLimb.LeftForearm,
      SkeletonLimb.LeftThigh,
      SkeletonLimb.LeftShin,
      SkeletonLimb.LeftFoot,
      SkeletonLimb.RightArm,
      SkeletonLimb.RightForearm,
      SkeletonLimb.RightThigh,
      SkeletonLimb.RightShin,
      SkeletonLimb.RightFoot,
    ];

    // generic approach(bad):
    // using reduce as a map + filter to avoid the extra array
    // return Object.keys(SkeletonLimb).reduce(
    //   /** @type {(acc: number[], field: string) => number[]} */
    //   (acc, field) => {
    //     // enum has static values for indices and names, we want to filter only the numbers
    //     let limb = SkeletonLimb[field];
    //     if (typeof limb === 'number') {
    //       acc.push(limb);
    //     }
    //     return acc;
    //   },
    //   /** @type {number[]}*/
    //   []
    // );
  }

  function limbTranslationKey(enumName) {
    if (isLeftBodyPart(enumName)) {
      return `editor.body.${enumName.substring(4)}`.toLowerCase();
    } else if (isRightBodyPart(enumName)) {
      return `editor.body.${enumName.substring(5)}`.toLowerCase();
    } else {
      return `editor.body.${enumName}`.toLowerCase();
    }
  }

  /** @param {string} enumName */
  function isLeftBodyPart(enumName) {
    return enumName.startsWith('Left');
  }

  /** @param {string} enumName */
  function isRightBodyPart(enumName) {
    return enumName.startsWith('Right');
  }
</script>

<div class="container">
  <div class="name">
    <input
      name="pose-name"
      class="name-input"
      bind:value={poseName}
      oninput={() => updateName()}
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

  <div class="right weights">
    {#each weightedLimbs as limb, index}
      {#if isRightBodyPart(limb.name)}
        <div class="body-part">
          <label for={limb.name}>{$t(limbTranslationKey(limb.name))}</label>
          <input
            name={limb.name}
            type="number"
            class="weight-input"
            bind:value={limb.weight}
            onchange={() => updateAvatar(index)}
            placeholder="Weight"
            min="0.0"
          />
        </div>
      {/if}
    {/each}
  </div>

  <div class="left weights">
    {#each weightedLimbs as limb, index}
      {#if isLeftBodyPart(limb.name)}
        <div class="body-part">
          <label for={limb.name}>{$t(limbTranslationKey(limb.name))}</label>
          <input
            name={limb.name}
            type="number"
            class="weight-input"
            bind:value={limb.weight}
            onchange={() => updateAvatar(index)}
            placeholder="Weight"
            min="0.0"
          />
        </div>
      {/if}
    {/each}
  </div>

  <div class="center weights">
    {#each weightedLimbs as limb, index}
      {#if !isLeftBodyPart(limb.name) && !isRightBodyPart(limb.name)}
        <div class="body-part">
          <label for={limb.name}>{$t(limbTranslationKey(limb.name))}</label>
          <input
            name={limb.name}
            type="number"
            class="weight-input"
            bind:value={limb.weight}
            onchange={() => updateAvatar(index)}
            placeholder="Weight"
            min="0.0"
          />
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-areas:
      'right avatar left'
      'right name left'
      'right center left';
  }
  .name {
    grid-area: name;
  }
  .left {
    grid-area: left;
  }
  .right {
    grid-area: right;
  }
  .center {
    grid-area: center;
  }
  .avatar {
    grid-area: avatar;
    border: solid 1px var(--theme-neutral-gray);
    border-radius: 50%;
  }
  .weights {
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
    font-size: var(--font-normal);
  }
  .name-input {
    border: none;
    width: 100%;
    text-align: center;
    font-size: var(--font-normal);
  }

  @media (max-width: 600px) {
    .container {
      grid-template-areas:
        'avatar avatar'
        'name name'
        'right left'
        'center center';
    }
  }
</style>
