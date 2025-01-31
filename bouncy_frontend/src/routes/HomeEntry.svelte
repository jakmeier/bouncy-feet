<script>
  import Area from '$lib/components/ui/Area.svelte';
  import {
    LEFT_RIGHT_COLORING,
    ORANGE_COLORING,
    ZEBRA_COLORING,
  } from '$lib/constants';
  import DanceAnimation from './DanceAnimation.svelte';
  import PathwaySuggestion from './PathwaySuggestion.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let featuredSteps;
  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper} */
  export let dance;

  /**
   * @type {number | undefined}
   */
  let selectedStep = undefined;
  let trainingsWidth = 250;
  $: trainingWidth = (trainingsWidth - 60) / 3;
</script>

<div class="container">
  {#if dance}
    <DanceAnimation
      size={trainingsWidth}
      {dance}
      style={ZEBRA_COLORING}
      beatDelay={3}
      hiddenBeats={3}
    />
  {:else}
    <Area
      width="{trainingsWidth}px"
      height="{trainingsWidth}px"
      borderRadius="20px"
      borderWidth="0px"
      backgroundColor="var(--theme-main)"
    ></Area>
  {/if}

  <div class="training">
    <PathwaySuggestion
      size={trainingWidth}
      step={featuredSteps[0]}
      style={ORANGE_COLORING}
      title={'record.train-dance-prefix'}
      fitness={100}
      xp={100}
      min={10}
      coach={'juhwang'}
    />
  </div>

  <div class="new-skill">
    <PathwaySuggestion
      size={trainingWidth}
      step={featuredSteps[1]}
      style={LEFT_RIGHT_COLORING}
      title={'record.learn-dance-prefix'}
      fitness={50}
      xp={50}
      min={3}
      coach={'chorok'}
    />
  </div>
  <a href="./coach/chorok"> </a>
</div>

<style>
  .container {
    position: relative;
  }

  .training,
  .new-skill {
    position: absolute;
    margin: 5px;
    padding: 5px;
    font-size: 22px;
    border: dotted var(--theme-accent);
    background-color: var(--theme-accent-light);
    border-radius: 10px;
    color: var(--theme-accent-dark);
    width: 40%;

    animation: levitate 1.5s infinite ease-in-out 0s alternate;
  }

  .training {
    top: 0;
    left: -20%;
  }

  .new-skill {
    right: -20%;
    top: 25%;
    animation-direction: alternate-reverse;
  }

  .shine {
    position: absolute;
    opacity: 0;
    top: 0;
    left: 0;

    width: 30%;
    height: 100%;

    transform: skew(-15deg, 0deg);
    animation: shine 5s;
    animation-iteration-count: infinite;
    animation-delay: 1s;

    background: linear-gradient(
      to right,
      rgba(255, 255, 255, 0) 0%,
      rgba(255, 255, 255, 0.6) 30%,
      rgba(255, 255, 255, 0.85) 50%,
      rgba(255, 255, 255, 0.6) 70%,
      rgba(255, 255, 255, 0) 100%
    );
  }

  @keyframes shine {
    0%,
    68% {
      left: 0;
      opacity: 0;
    }
    82% {
      opacity: 0.5;
    }
    94% {
      opacity: 0;
    }
    100% {
      left: 82%;
    }
  }

  @keyframes levitate {
    0% {
      transform: translateY(0px);
    }

    100% {
      transform: translateY(8px);
    }
  }
</style>
