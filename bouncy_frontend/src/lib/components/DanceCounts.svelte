<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { Skeleton, StepWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  /**
   * @typedef {Object} Props
   * @property {StepWrapper[]} steps
   * @property {any} [highlightedStep]
   * @property {any} [markedPoseIndex]
   */

  /** @type {Props} */
  let {
    steps,
    highlightedStep = $bindable(-1),
    markedPoseIndex = -1,
  } = $props();

  let innerWidth = $state(300);

  let totalSubbeats = $derived(steps.reduce((a, b) => a + b.subbeats, 0));
  const posesPerRow = $derived(Math.min(8, totalSubbeats));
  let poseWidth = $derived(innerWidth / posesPerRow);
  /** @type {number[]} */
  let stepTransitions = $derived(
    steps.reduce(
      (acc, step) => {
        const prev = acc.length === 0 ? 0 : acc[acc.length - 1];
        acc.push(prev + step.subbeats);
        return acc;
      },
      /** @type {number[]} */
      []
    )
  );
  /** @type {Skeleton[]} */
  let poses = steps.flatMap((step) =>
    new Array(step.subbeats).fill(0).map((_, beat) => step.skeleton(beat))
  );

  function count(subbeat) {
    if (subbeat % 2 === 1) {
      return '+';
    } else {
      return (subbeat % 8) / 2 + 1;
    }
  }

  function selectBeat(beat) {
    highlightedStep = stepTransitions.findIndex((t) => t > beat);
  }
</script>

<div
  class="poses"
  bind:clientWidth={innerWidth}
  style="--num-beats: {posesPerRow}"
>
  {#each { length: totalSubbeats } as _, beat}
    <!-- <Pose /> -->
    <div
      class="pose"
      style="width: {poseWidth}px"
      onclick={() => selectBeat(beat)}
      role="button"
      tabindex={0}
      onkeydown={(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          selectBeat(beat);
        }
      }}
    >
      <div
        class="count"
        class:marked={beat === markedPoseIndex % totalSubbeats}
      >
        {count(beat)}
      </div>
      {#if stepTransitions.includes(beat)}
        <div class="step-transition"></div>
      {/if}
      {#if beat >= (stepTransitions[highlightedStep - 1] || 0) && beat < stepTransitions[highlightedStep]}
        <div class="highlighted"></div>
      {/if}
      <div class="avatar">
        <Svg width={200} height={200} orderByZ>
          <SvgAvatar skeleton={poses[beat]} width={200} height={200}
          ></SvgAvatar>
        </Svg>
      </div>
    </div>
  {/each}
</div>

<style>
  .poses {
    display: grid;
    grid-template-columns: repeat(var(--num-beats), 1fr);
  }
  .pose {
    position: relative;
    width: 100px;
    padding-bottom: 30px;
  }
  .count {
    position: relative;
    z-index: 4;
    text-align: center;
  }
  .marked {
    font-weight: 800;
    color: var(--theme-neutral-white);
    background-color: var(--theme-main-dark);
    border-radius: 5px;
  }
  .step-transition {
    position: absolute;
    z-index: 2;
    left: 0;
    top: 0;
    background-color: var(--theme-neutral-light);
    height: 90%;
    width: 10px;
    border-radius: 5px;
  }
  .highlighted {
    position: absolute;
    z-index: 1;
    left: 5px;
    top: 0;
    height: 90%;
    width: 100%;
    background-color: var(--theme-neutral-light);
  }
  .avatar {
    position: relative;
    z-index: 3;
  }
</style>
