<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { DanceWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  
  /**
   * @typedef {Object} Props
   * @property {DanceWrapper} dance
   * @property {any} [highlightedStep]
   * @property {any} [markedPoseIndex]
   */

  /** @type {Props} */
  let { dance, highlightedStep = $bindable(-1), markedPoseIndex = -1 } = $props();

  let innerWidth = $state(300);

  let subbeat = $derived(dance.subbeats);
  let poseWidth = $derived(innerWidth / 8);
  /** @type {number[]} */
  let stepTransitions = $derived(dance.steps().reduce(
    (acc, step) => {
      const prev = acc.length === 0 ? 0 : acc[acc.length - 1];
      acc.push(prev + step.subbeats);
      return acc;
    },
    /** @type {number[]} */
    []
  ));

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

<div class="poses" bind:clientWidth={innerWidth}>
  {#each { length: subbeat } as _, beat}
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
      <div class="count" class:marked={beat === markedPoseIndex % subbeat}>
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
          <SvgAvatar
            skeleton={dance.skeleton(beat)}
            width={200}
            height={200}
            style={LEFT_RIGHT_COLORING_LIGHT}
          ></SvgAvatar>
        </Svg>
      </div>
    </div>
  {/each}
</div>

<style>
  .poses {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
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
    color: white;
    background-color: var(--theme-neutral-light);
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
