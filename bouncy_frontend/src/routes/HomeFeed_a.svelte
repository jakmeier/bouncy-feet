<script>
  import { t } from '$lib/i18n.js';
  import { onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import { BOLD_MAIN_THEME_COLORING, WHITE_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper[]} */
  export let featuredDances;

  let swapBackgroundColor = 'var(--theme-neutral-white)';
  onMount(() => {
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-main)';
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
</script>

<div class="title">
  <img class="logo" src="{base}/icons/logo.svg" alt="Bouncy Feet Logo" />
  <h1 translate="no">
    {$t('home.title')}
  </h1>
</div>

<div class="dark-stripe">
  <div class="dancers" style="grid-template-columns: repeat({7}, 1fr);">
    {#each { length: 4 } as _}
      <DanceAnimation dance={dance(0)} style={WHITE_COLORING} showOverflow />
      <div></div>
    {/each}
  </div>
</div>

<div class="light-box slogan">{$t('home.slogan0')}</div>

<div class="dark-stripe">
  <div class="dancers" style="grid-template-columns: repeat({3}, 1fr);">
    <div></div>
    <DanceAnimation
      dance={dance(0)}
      style={BOLD_MAIN_THEME_COLORING}
      showOverflow
    />
    <div></div>
  </div>
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

<div class="dark-stripe">
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

<div class="dark-stripe">
  <div
    class="dancers"
    style="grid-template-columns: repeat({9}, 1fr); margin-left: -50px;"
  >
    {#each { length: 9 } as _}
      <DanceAnimation dance={dance(3)} style={WHITE_COLORING} showOverflow />
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
    margin: 10px 0 25px;
  }
  h1 {
    text-align: center;
    text-shadow: var(--theme-neutral-white) 0px 0px 11px;
    font-size: 45px;
  }
  .logo {
    max-width: 128px;
  }
  .dancers {
    display: grid;
  }
  .dark-stripe {
    background-color: var(--theme-neutral-dark);
    /* padding and margin make the strip expand outside the view on mobile */
    padding: 10px 50px;
    margin: -10px -50px;
    /* on wide screens, the stripe should end with a nice rounding */
    border-radius: 10px;
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
  .slogan {
    box-shadow: none;
    max-width: 400px;
    margin: auto;
    text-align: center;
    font-size: 50px;
    background-color: var(--theme-main);
    color: var(--theme-neutral-white);
    z-index: 1;
    position: relative;
  }
  button {
    margin: 5px;
  }
</style>
