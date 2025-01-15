<script>
  import { t } from '$lib/i18n.js';
  import { onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import { BOLD_MAIN_THEME_COLORING, WHITE_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import Step from './collection/Step.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper[]} */
  export let featuredDances;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper} */
  export let featuredStep;

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
</script>

<div class="title">
  <img
    class="logo"
    src="{base}/icons/new-logo-black.svg"
    alt="Bouncy Feet Logo"
  />
  <h1 class="logo-font" translate="no">BouncyFeet</h1>
</div>

<div
  class="dark stripe rotate-left bounce"
  style="--time-between-moves: {$timeBetweenMoves}ms; --animation-delay: {animationDelay}s"
>
  <div class="dancers" style="grid-template-columns: repeat({7}, 1fr);">
    {#each { length: 4 } as _}
      <DanceAnimation
        dance={dance(0)}
        style={WHITE_COLORING}
        beat={beatCounter}
        {animationTime}
        showOverflow
      />
      <div></div>
    {/each}
  </div>
</div>

<div class="light-box focus-card">
  <div>
    {$t('home.test0')}:
  </div>

  <div class="space">
    <Step
      size={300}
      step={featuredStep}
      poseIndex={$beatCounter / 2}
      animationTime={$timeBetweenMoves}
    />
    <div class="space">
      {featuredStep.name}
    </div>

    <!-- Mock up -->
    <button class="light wide"> {$t('home.go-button')} </button>
  </div>
</div>

<div class="green stripe rotate-right">
  <div style="min-height: 100px"></div>
</div>

<div class="light-box">
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

<div class="orange stripe rotate-left">
  <div style="height: 40px;"></div>
</div>

<div class="light-box">
  <div>
    {$t('home.go-to-github')}
  </div>
  <div class="centered">
    <a href="https://github.com/jakmeier/bouncy-feet/issues">
      <button class="light"> GitHub </button>
    </a>
  </div>
</div>

<div
  class="dark stripe bounce"
  style="--time-between-moves: {$timeBetweenMoves}ms; --animation-delay: {animationDelay}s"
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

<div class="light-box">
  <div>
    <i>
      {$t('home.version-label')}:
      {$versionString}
    </i>
  </div>
</div>

<style>
  .title {
    display: flex;
    justify-content: center;
    margin: -10px -10px;
    padding: 0 5px 5px;
    background-color: var(--theme-main);
    color: var(--theme-neutral-dark);
    /* box-shadow: var(--theme-neutral-dark) 0px 0px 11px; */
  }
  h1 {
    text-align: center;
    font-size: 45px;
  }
  .logo {
    max-width: 128px;
  }
  .dancers {
    display: grid;
  }
  .stripe {
    /* padding and margin make the strip expand outside the view on mobile */
    padding: 10px 50px;
    margin: -10px -50px;
    /* on wide screens, the stripe should end with a nice rounding */
    border-radius: 10px;
  }
  .dark {
    background-color: var(--theme-neutral-dark);
  }
  .green {
    background-color: var(--theme-main);
  }
  .orange {
    background-color: var(--theme-accent);
  }
  .rotate-left {
    rotate: -8deg;
  }
  .rotate-right {
    rotate: 13deg;
  }

  .light-box {
    padding: 20px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    margin: 5px 0;
    z-index: 1;
    position: relative;
    box-shadow: var(--theme-neutral-dark) 0px 0px 11px;
    text-align: center;
  }
  .centered {
    margin-top: 15px;
    text-align: center;
  }
  .focus-card {
    max-width: 400px;
    margin: auto;
    padding: 100px 20px;
    text-align: center;
    font-size: 32px;
    background-color: var(--theme-neutral-light);
    /* color: var(--theme-neutral-white); */
    z-index: 1;
    position: relative;
    /* rotate: -30deg; */
  }
  button {
    margin: 5px;
  }

  .logo-font {
    font-family: 'Quicksand', serif;
    font-optical-sizing: auto;
    font-weight: 700;
    font-style: normal;
    letter-spacing: -2px;
  }

  .space {
    margin: 60px 10px;
  }

  .bounce {
    animation: bounce var(--time-between-moves) infinite ease-in-out
      var(--animation-delay);
  }

  @keyframes bounce {
    0%,
    100% {
      transform: translateY(0);
    }
    90% {
      transform: translateY(-2px);
    }
  }
</style>
