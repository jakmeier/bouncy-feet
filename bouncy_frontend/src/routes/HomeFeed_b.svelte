<script>
  import { t } from '$lib/i18n.js';
  import { getContext, onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import { WHITE_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import Area from '$lib/components/ui/Area.svelte';
  import Pose from '$lib/components/Pose.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper[]} */
  export let featuredDances;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let featuredSteps;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper} */
  export let idleStep;

  const localCollectionCtx = getContext('localCollection');
  /** @type {Readable<PoseWrapper[]>} */
  const poses = localCollectionCtx.poses;

  let swapBackgroundColor = 'var(--theme-neutral-white)';
  onMount(() => {
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-neutral-white)';
  });
  onDestroy(() => {
    $backgroundColor = swapBackgroundColor;
  });

  /**
   * @param {number} i
   * @returns {import("$lib/instructor/bouncy_instructor").DanceWrapper}
   */
  function dance(i) {
    return featuredDances[i % featuredDances.length];
  }

  $: animationTime = $timeBetweenMoves * 0.85;
  let animationDelay = 99;
  beatCounter.subscribe((counter) => {
    if (counter > 0) {
      animationDelay = 0;
    }
  });

  /**
   * @type {number | undefined}
   */
  let selectedStep = undefined;
  $: bigAvatarStep =
    selectedStep !== undefined ? featuredSteps[selectedStep] : undefined;
  let trainingsWidth = 300;
  $: trainingWidth = (trainingsWidth - 60) / 3;
</script>

<div class="title">
  <img
    class="logo"
    src="{base}/icons/gradient-icon.svg"
    alt="Bouncy Feet Logo"
  />
</div>

<div class="light-box focus-card">
  <div>
    {$t('home.test0')}
  </div>

  <div class="space">
    {#if bigAvatarStep}
      <AnimatedStep size={300} step={bigAvatarStep} />
    {:else if $poses[0]}
      <Area
        width="300px"
        height="300px"
        borderRadius="20px"
        borderWidth="0px"
        backgroundColor="var(--theme-neutral-gray)"
      >
        <Pose size={300} pose={idleStep.poses()[0]} />
      </Area>
    {/if}

    <div class="trainings" bind:clientWidth={trainingsWidth}>
      {#each featuredSteps as step, i}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          class="training"
          class:marked={selectedStep === i}
          on:click={() => (selectedStep = i)}
        >
          <AnimatedStep size={trainingWidth} {step} />
        </div>
      {/each}
    </div>

    <!-- Mock up -->
    <button class="light wide"> {$t('home.go-button')} </button>
  </div>
</div>

<div class="light-box space">
  <div>
    {$t('home.description3')}
  </div>
  <div class="centered">
    <a href="./collection">
      <button class="light wide"> {$t('home.collection-button')} </button>
    </a>
    <a href="./courses">
      <button class="light wide"> {$t('home.courses-button')} </button>
    </a>
  </div>
  <p>
    <i>
      {$t('home.alpha-disclaimer')}
    </i>
  </p>
</div>

<div class="light-box space">
  <div>
    {$t('home.go-to-github')}
  </div>
  <div class="centered">
    <a href="https://github.com/jakmeier/bouncy-feet/issues">
      <button class="light"> GitHub </button>
    </a>
  </div>
</div>

<div class="light-box rotated">
  <img
    class="logo rotated"
    src="{base}/icons/bouncyfeet.png"
    alt="Bouncy Feet Logo"
  />
</div>

<div class="light-box">
  <div>
    <i>
      {$t('home.version-label')}:
      {$versionString}
    </i>
  </div>
</div>

<div
  class="dark stripe bounce"
  style="--time-between-moves: {$timeBetweenMoves *
    2}ms; --animation-delay: {animationDelay}s"
>
  <div
    class="dancers"
    style="grid-template-columns: repeat({9}, 1fr); margin-left: -50px;"
  >
    {#each { length: 9 } as _}
      <DanceAnimation
        dance={dance(3)}
        style={WHITE_COLORING}
        beat={beatCounter}
        {animationTime}
        showOverflow
      />
    {/each}
  </div>
</div>

<style>
  .title {
    position: relative;
    display: flex;
    justify-content: center;
    margin: -10px -100%;
    padding: 0 5px 5px;
    background-color: var(--theme-neutral-white);
    color: var(--theme-neutral-dark);
    z-index: 1;
    height: 10vh;
    /* display: grid; */
    /* grid-template-columns: 1fr; */
    /* justify-items: center; */
    /* box-shadow: var(--theme-neutral-dark) 0px 0px 11px; */
    /* height: 100vh; */
  }
  img {
    margin: 2rem;
    max-width: 30%;
  }
  .title img.logo {
    /* position: absolute; */
    /* top: 15vh; */
    height: 90%;
    margin: auto;
  }

  .dancers {
    display: grid;
  }
  .stripe {
    /* padding and margin make the strip expand outside the view on mobile */
    padding: 10px 100%;
    margin: 10px -100%;
    /* on wide screens, the stripe should end with a nice rounding */
    border-radius: 10px;
    position: relative;
  }
  .dark {
    background-color: var(--theme-neutral-dark);
  }

  .light-box {
    padding: 20px;
    /* background-color: var(--theme-neutral-light); */
    border-radius: 10px;
    margin: 5px 0;
    z-index: 1;
    position: relative;
    /* box-shadow: var(--theme-neutral-dark) 0px 0px 11px; */
    text-align: center;
  }
  .centered {
    margin-top: 15px;
    text-align: center;
  }
  .light-box.rotated {
    height: 0px;
    overflow: visible;
    rotate: 90deg;
    display: flex;
  }
  .rotated img {
    max-height: min(10vw, 100px);
    max-width: unset;
    align-self: start;
    transform: translate(-8vw, calc(50vw - min(10vw, 100px)));
    margin: 0;
  }
  .focus-card {
    max-width: 400px;
    margin: auto;
    padding: 100px 20px;
    text-align: center;
    font-size: 32px;
    /* background-color: var(--theme-neutral-light); */
    /* color: var(--theme-neutral-white); */
    z-index: 1;
    position: relative;
    /* rotate: -30deg; */
  }
  button {
    margin: 5px;
  }

  .space {
    margin: 60px 10px;
  }

  .bounce {
    animation: bounce var(--time-between-moves) infinite ease-in-out
      var(--animation-delay);
  }

  .trainings {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
  }

  .training {
    margin: 5px;
    padding: 5px;
  }

  .marked {
    background-color: var(--theme-main-dark);
    border-radius: 5px;
  }

  @keyframes bounce {
    25%,
    75% {
      transform: translateY(0);
    }
    0% {
      transform: translateY(-2px);
    }
    50% {
      transform: translateY(-5px);
    }
  }
</style>
