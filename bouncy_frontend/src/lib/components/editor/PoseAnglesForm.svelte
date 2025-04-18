<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { t } from '$lib/i18n';
  import {
    PoseWrapper,
    Skeleton,
    SkeletonWrapper,
    SkeletonLimb,
    SkeletonPoint,
  } from '$lib/instructor/bouncy_instructor';
  import Svg from '../avatar/Svg.svelte';
  import SvgAvatar from '../avatar/SvgAvatar.svelte';
  import AngleInput from '../ui/AngleInput.svelte';
  import Button from '../ui/Button.svelte';
  import NumberSlider from '../ui/NumberSlider.svelte';

  let bodyLimbs = $state([
    { name: 'editor.body.arm', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.arm', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.forearm', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.forearm', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.thigh', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.thigh', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.shin', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.shin', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.foot', angle: 0, isKey: false, weight: null },
    { name: 'editor.body.foot', angle: 0, isKey: false, weight: null },
  ]);

  let bodyParts = $state([
    // TODO: shoulder and hips should also be moveable (currently fixed by pose model)
    // { name: 'editor.body.shoulder', i: SkeletonPoint.LeftShoulder, z: 0.0 },
    // { name: 'editor.body.shoulder', i: SkeletonPoint.RightShoulder, z: 0.0 },
    { name: 'editor.body.elbow', i: SkeletonPoint.LeftElbow, z: 0.0 },
    { name: 'editor.body.elbow', i: SkeletonPoint.RightElbow, z: 0.0 },
    { name: 'editor.body.wrist', i: SkeletonPoint.LeftWrist, z: 0.0 },
    { name: 'editor.body.wrist', i: SkeletonPoint.RightWrist, z: 0.0 },
    // { name: 'editor.body.hip', i: SkeletonPoint.LeftHip, z: 0.0 },
    // { name: 'editor.body.hip', i: SkeletonPoint.RightHip, z: 0.0 },
    { name: 'editor.body.knee', i: SkeletonPoint.LeftKnee, z: 0.0 },
    { name: 'editor.body.knee', i: SkeletonPoint.RightKnee, z: 0.0 },
    { name: 'editor.body.heel', i: SkeletonPoint.LeftHeel, z: 0.0 },
    { name: 'editor.body.heel', i: SkeletonPoint.RightHeel, z: 0.0 },
    { name: 'editor.body.ankle', i: SkeletonPoint.LeftAnkle, z: 0.0 },
    { name: 'editor.body.ankle', i: SkeletonPoint.RightAnkle, z: 0.0 },
    { name: 'editor.body.toes', i: SkeletonPoint.LeftToes, z: 0.0 },
    { name: 'editor.body.toes', i: SkeletonPoint.RightToes, z: 0.0 },
  ]);

  let bodyShift = $state({ x: 0, y: 0 });

  /** @type{Skeleton | undefined} */
  let skeleton = $state(Skeleton.resting(false));
  /** @type{Skeleton} */
  let sideSkeleton = $state(Skeleton.resting(true));
  /** @type{PoseWrapper | undefined} */
  let pose = $state();

  /**
   * @typedef {Object} Props
   * @property {any} [onChange]
   */

  /** @type {Props} */
  let { onChange = (pose) => {} } = $props();

  /** @param {SkeletonWrapper} inputSkeleton */
  export function loadSkeleton(inputSkeleton) {
    loadPose(inputSkeleton.pose());
  }

  /** @param {PoseWrapper} inputPose */
  export function loadPose(inputPose) {
    pose = inputPose;
    updateAnglesFromPose(pose);
    updateZFromPose(pose);
    bodyShift.x = pose.xShift;
    bodyShift.y = pose.yShift;
  }

  /** @returns {PoseWrapper | undefined} */
  export function readPose() {
    return pose;
  }

  /** @param {PoseWrapper} pose */
  function updateAnglesFromPose(pose) {
    skeleton = pose.skeleton();
    sideSkeleton = pose.sideSkeleton();

    bodyLimbs[0].angle = pose.getAngle(SkeletonLimb.LeftArm) - 90.0;
    bodyLimbs[1].angle = pose.getAngle(SkeletonLimb.RightArm) - 90.0;
    bodyLimbs[2].angle = pose.getAngle(SkeletonLimb.LeftForearm) - 90.0;
    bodyLimbs[3].angle = pose.getAngle(SkeletonLimb.RightForearm) - 90.0;
    bodyLimbs[4].angle = pose.getAngle(SkeletonLimb.LeftThigh) - 90.0;
    bodyLimbs[5].angle = pose.getAngle(SkeletonLimb.RightThigh) - 90.0;
    bodyLimbs[6].angle = pose.getAngle(SkeletonLimb.LeftShin) - 90.0;
    bodyLimbs[7].angle = pose.getAngle(SkeletonLimb.RightShin) - 90.0;
    bodyLimbs[8].angle = pose.getAngle(SkeletonLimb.LeftFoot) - 90.0;
    bodyLimbs[9].angle = pose.getAngle(SkeletonLimb.RightFoot) - 90.0;
  }

  /** @param {PoseWrapper} pose */
  function updateZFromPose(pose) {
    bodyParts[0].z = pose.getZ(SkeletonPoint.LeftElbow);
    bodyParts[1].z = pose.getZ(SkeletonPoint.RightElbow);
    bodyParts[2].z = pose.getZ(SkeletonPoint.LeftWrist);
    bodyParts[3].z = pose.getZ(SkeletonPoint.RightWrist);
    bodyParts[4].z = pose.getZ(SkeletonPoint.LeftKnee);
    bodyParts[5].z = pose.getZ(SkeletonPoint.RightKnee);
    bodyParts[6].z = pose.getZ(SkeletonPoint.LeftHeel);
    bodyParts[7].z = pose.getZ(SkeletonPoint.RightHeel);
    bodyParts[8].z = pose.getZ(SkeletonPoint.LeftAnkle);
    bodyParts[9].z = pose.getZ(SkeletonPoint.RightAnkle);
    bodyParts[9].z = pose.getZ(SkeletonPoint.LeftToes);
    bodyParts[9].z = pose.getZ(SkeletonPoint.RightToes);
  }

  function updateAvatarAngles() {
    if (pose) {
      // use setter on skeleton to avoid copying intermediate objects on the
      // field getters generated by wasm bindgen
      pose.setAngle(SkeletonLimb.LeftArm, bodyLimbs[0].angle);
      pose.setAngle(SkeletonLimb.RightArm, bodyLimbs[1].angle);
      pose.setAngle(SkeletonLimb.LeftForearm, bodyLimbs[2].angle);
      pose.setAngle(SkeletonLimb.RightForearm, bodyLimbs[3].angle);
      pose.setAngle(SkeletonLimb.LeftThigh, bodyLimbs[4].angle);
      pose.setAngle(SkeletonLimb.RightThigh, bodyLimbs[5].angle);
      pose.setAngle(SkeletonLimb.LeftShin, bodyLimbs[6].angle);
      pose.setAngle(SkeletonLimb.RightShin, bodyLimbs[7].angle);
      pose.setAngle(SkeletonLimb.LeftFoot, bodyLimbs[8].angle);
      pose.setAngle(SkeletonLimb.RightFoot, bodyLimbs[9].angle);

      updateSkeleton();
    }
  }

  function updateAvatarZ() {
    if (pose) {
      // use setter on skeleton to avoid copying intermediate objects on the
      // field getters generated by wasm bindgen
      for (let point of bodyParts) {
        pose.setZ(point.i, point.z);
      }

      updateSkeleton();
    }
  }

  function updateBodyShift() {
    if (pose) {
      pose.xShift = bodyShift.x;
      pose.yShift = bodyShift.y;
      updateSkeleton();
    }
  }

  function updateSkeleton() {
    if (pose) {
      skeleton = pose.skeleton();
      sideSkeleton = pose.sideSkeleton();
      onChange(pose);
    }
  }
</script>

<div class="container">
  <div class="left input-group">
    {#each bodyLimbs as part, index}
      {#if index % 2 === 0}
        <div class="body-part">
          <label for="left-{index}">{$t(part.name)}</label>
          <AngleInput
            name="left-{index}"
            bind:value={part.angle}
            onChange={updateAvatarAngles}
          ></AngleInput>
        </div>
      {/if}
    {/each}
  </div>

  <div class="right input-group">
    {#each bodyLimbs as part, index}
      {#if index % 2 === 1}
        <div class="body-part">
          <label for="right-{index}">{$t(part.name)}</label>
          <AngleInput
            name="right-{index}"
            bind:value={part.angle}
            onChange={updateAvatarAngles}
            thumbColor={'var(--theme-accent)'}
          ></AngleInput>
        </div>
      {/if}
    {/each}
  </div>

  <div class="avatar">
    <Svg width={200} height={200} orderByZ>
      {#if skeleton}
        <SvgAvatar {skeleton} width={200} height={200}></SvgAvatar>
      {/if}
    </Svg>
  </div>

  {#if pose}
    <div class="direction">
      <!-- {PoseDirection[pose?.direction]} -->
      <Button
        class="short"
        symbolSize={29}
        symbol="sync"
        on:click={() => {
          pose.setDirection((pose.direction + 1) % 2);
          updateSkeleton();
        }}
      ></Button>
    </div>

    <div class="body-turn">
      <div class="turn">
        <label for="turn-shoulder">{$t('editor.body.shoulder')}</label>
        <input
          name="turn-shoulder"
          type="number"
          class="turn-input angle-input"
          bind:value={pose.turnShoulder}
          onchange={updateSkeleton}
          placeholder={$t('editor.body.shoulder')}
          min={-45}
          max={45}
        />
      </div>

      <div class="turn">
        <label for="turn-hip">{$t('editor.body.hip')}</label>
        <input
          name="turn-hip"
          type="number"
          class="turn-input angle-input"
          bind:value={pose.turnHip}
          onchange={updateSkeleton}
          placeholder={$t('editor.body.hip')}
          min={-45}
          max={45}
        />
      </div>
    </div>

    <div class="shift">
      <div class="turn">
        <label for="xshift">X</label>
        <input
          name="xshift"
          type="number"
          class="turn-input angle-input"
          bind:value={bodyShift.x}
          onchange={updateBodyShift}
          min={-4}
          max={4}
          step={0.0125}
        />
      </div>

      <div class="turn">
        <label for="yshift">Y</label>
        <input
          name="yshift"
          type="number"
          class="turn-input angle-input"
          bind:value={bodyShift.y}
          onchange={updateBodyShift}
          min={-4}
          max={4}
          step={0.0125}
        />
      </div>
    </div>
  {/if}
</div>

<!-- Side view -->
<div class="container">
  <div class="left input-group">
    {#each bodyParts as part, index}
      {#if index % 2 === 0}
        <div class="body-part">
          <label for="left-{index}">{$t(part.name)}</label>
          <NumberSlider
            name="left-{index}"
            bind:value={part.z}
            onChange={updateAvatarZ}
            min={-5.0}
            max={5.0}
            decimals={1}
          ></NumberSlider>
        </div>
      {/if}
    {/each}
  </div>

  <div class="right input-group">
    {#each bodyParts as part, index}
      {#if index % 2 === 1}
        <div class="body-part">
          <label for="right-{index}">{$t(part.name)}</label>
          <NumberSlider
            name="right-{index}"
            bind:value={part.z}
            onChange={updateAvatarZ}
            min={-5.0}
            max={5.0}
            decimals={1}
            thumbColor={'var(--theme-accent)'}
          ></NumberSlider>
        </div>
      {/if}
    {/each}
  </div>

  <div class="avatar">
    <Svg width={200} height={200} orderByZ>
      <SvgAvatar
        skeleton={sideSkeleton}
        width={200}
        height={200}
        style={LEFT_RIGHT_COLORING_LIGHT}
      ></SvgAvatar>
    </Svg>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-areas:
      'right-values direction left-values'
      'right-values avatar left-values'
      'right-values avatar left-values'
      'right-values avatar left-values'
      'right-values body-turn left-values'
      'right-values shift left-values';
    gap: 8px;
    justify-items: center;
  }
  .left {
    grid-area: left-values;
  }
  .right {
    grid-area: right-values;
  }
  .avatar {
    grid-area: avatar;
  }
  .direction {
    grid-area: direction;
  }
  .body-turn {
    grid-area: body-turn;
  }
  .shift {
    grid-area: shift;
  }
  .input-group {
    padding: 5px;
    width: 90%;
  }
  .body-part {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 90%;
    margin: 5px auto;
    padding: 10px 5px;
  }
  .left .body-part {
    background-color: var(--theme-accent);
    color: var(--theme-neutral-black);
  }
  .right .body-part {
    background-color: var(--theme-main);
    color: var(--theme-neutral-black);
  }
  .angle-input {
    width: 100%;
    text-align: center;
  }
  .turn-input {
    font-size: var(--font-normal);
  }
  .body-turn,
  .shift {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin: 10px;
  }

  @media (max-width: 600px) {
    .container {
      grid-template-areas:
        'direction direction'
        'body-turn body-turn'
        'avatar avatar'
        'right-values left-values';
    }
  }
</style>
