<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { DanceWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  /** @type {DanceWrapper} */
  export let dance;
  export let highlightedStep = -1;
  export let markedPoseIndex = -1;

  let innerWidth = 300;

  $: beats = dance.beats;
  $: poseWidth = innerWidth / 8;
  /** @type {number[]} */
  $: stepTransitions = dance.steps().reduce(
    (acc, step) => {
      const prev = acc.length === 0 ? 0 : acc[acc.length - 1];
      acc.push(prev + step.beats);
      return acc;
    },
    /** @type {number[]} */
    []
  );

  function count(beat) {
    if (beat % 2 === 1) {
      return '+';
    } else {
      return (beat % 8) / 2 + 1;
    }
  }

  function selectBeat(beat) {
    highlightedStep = stepTransitions.findIndex((t) => t > beat);
  }
</script>

<div class="poses" bind:clientWidth={innerWidth}>
  {#each { length: beats } as _, beat}
    <!-- <Pose /> -->
    <div
      class="pose"
      style="width: {poseWidth}px"
      on:click={() => selectBeat(beat)}
      role="button"
      tabindex={0}
      on:keydown={(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          selectBeat(beat);
        }
      }}
    >
      <div class="count" class:marked={beat === markedPoseIndex % beats}>
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
