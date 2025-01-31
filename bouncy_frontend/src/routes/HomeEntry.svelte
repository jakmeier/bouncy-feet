<script>
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import Pose from '$lib/components/Pose.svelte';
  import Area from '$lib/components/ui/Area.svelte';
  import { ORANGE_COLORING, ZEBRA_COLORING } from '$lib/constants';
  import { t } from '$lib/i18n';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';

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
    <div class=""></div>

    <div>{$t('record.train-dance-prefix')}</div>
    <AnimatedStep
      size={trainingWidth}
      step={featuredSteps[0]}
      style={ORANGE_COLORING}
    />
    <div class="bubbles">
      <div>+100 Fitness</div>
      <div>+100 XP</div>
      <div>10min</div>
    </div>
  </div>

  <div class="new-skill">
    <div class=""></div>
    <div>{$t('record.learn-dance-prefix')}</div>
    <AnimatedStep size={trainingWidth} step={featuredSteps[1]} />
    <div class="bubbles">
      <div>+1 Skill</div>
      <div>+50 XP</div>
      <div>3min</div>
    </div>
  </div>
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

  .bubbles div {
    background-color: var(--theme-accent);
    color: var(--theme-neutral-white);
    border-radius: 50vh;
    font-size: 18px;
    padding: 2px;
    margin: 2px;
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
