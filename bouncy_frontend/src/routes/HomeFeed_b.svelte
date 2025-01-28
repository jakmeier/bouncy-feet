<script>
  import { t } from '$lib/i18n.js';
  import { onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import {
    LEFT_RIGHT_COLORING,
    ORANGE_COLORING,
    WHITE_COLORING,
  } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import HomeEntry from './HomeEntry.svelte';
  import PathwayProgress from './PathwayProgress.svelte';
  import SpeechBubble from '$lib/components/ui/SpeechBubble.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper[]} */
  export let featuredDances;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let featuredSteps;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper} */
  export let idleStep;

  const entryDance = featuredDances.find(
    (dance) => dance.id === 'Home Animation (dev)'
  );

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
</div>

<div class="focus-card">
  <div class="light-box lime">
    <div class="speech-bubble">
      <SpeechBubble text={$t('home.test0')} position="bottom" width={'100%'} />
    </div>

    <div>
      <HomeEntry {featuredSteps} dance={entryDance}></HomeEntry>
    </div>
  </div>
</div>

<div class="space">
  <div class="light-box transparent">
    <h3>
      {$t('home.progress-title')}
    </h3>
  </div>
</div>

<div class="light-box">
  <PathwayProgress
    teacherName="V-Step Master"
    step={featuredSteps[1]}
    style={LEFT_RIGHT_COLORING}
    experience={700}
    skill={7}
    maxSkill={10}
    totalSteps={1703}
  />
</div>

<div class="light-box">
  <PathwayProgress
    teacherName="Running Man Coach"
    step={featuredSteps[0]}
    style={ORANGE_COLORING}
    experience={0}
    skill={0}
    maxSkill={10}
    totalSteps={0}
  />
</div>

<div class="space">
  <div class="light-box transparent">
    <div>
      {$t('home.go-to-github')}
    </div>
    <div class="centered">
      <a href="https://github.com/jakmeier/bouncy-feet/issues">
        <button class="light"> GitHub </button>
      </a>
    </div>
  </div>
</div>

<div class="rotated-outer">
  <img
    class="logo rotated"
    src="{base}/icons/bouncyfeet.png"
    alt="Bouncy Feet Logo"
  />
</div>

<div class="light-box transparent">
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
    background-color: var(--theme-neutral-white);
    background-blend-mode: saturation;

    border-radius: 10px;
    margin: 15px 5px;
    z-index: 1;
    position: relative;
    box-shadow: var(--theme-neutral-dark) 0px 0px 11px;
    text-align: center;
  }
  .lime {
    background-color: var(--theme-main);
  }
  .transparent {
    background-color: #e6f2efa0;
    box-shadow: none;
    margin: 32px;
  }
  .centered {
    margin-top: 15px;
    text-align: center;
  }
  .rotated-outer {
    height: 0px;
    overflow: visible;
    rotate: 90deg;
  }
  .rotated-outer img {
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
    width: fit-content;
    height: fit-content;
  }

  .space {
    margin: 60px 10px;
  }

  .bounce {
    animation: bounce var(--time-between-moves) infinite ease-in-out
      var(--animation-delay);
  }

  .speech-bubble {
    margin-bottom: 20px;
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
