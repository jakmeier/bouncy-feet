<script>
  import { t } from '$lib/i18n.js';
  import { onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import { BOLD_MAIN_THEME_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceInfo[]} */
  export let featuredDances;

  const bigNumDancers = 8;
  const midNumDancers = 5;
  const smallNumDancers = 3;

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
   * @returns {import("$lib/instructor/bouncy_instructor").DanceInfo}
   */
  function dance(i) {
    return featuredDances[i % featuredDances.length];
  }
</script>

<div class="title">
  <img class="logo" src="{base}/icons/logo.svg" alt="Bouncy Feet Logo" />
  <h1>
    {$t('home.title')}
  </h1>
</div>

<div class="dark-stripe">
  <div
    class="dancers"
    style="grid-template-columns: repeat({midNumDancers}, 1fr);"
  >
    {#each { length: midNumDancers } as _}
      <DanceAnimation
        dance={dance(0)}
        style={BOLD_MAIN_THEME_COLORING}
        showOverflow
      />
    {/each}
  </div>
</div>

<div class="light-box slogan">{$t('home.slogan0')}</div>

<div class="dark-stripe">
  <div
    class="dancers"
    style="grid-template-columns: repeat({smallNumDancers}, 1fr);"
  >
    {#each { length: smallNumDancers } as _}
      <DanceAnimation
        dance={dance(1)}
        style={BOLD_MAIN_THEME_COLORING}
        showOverflow
      />
    {/each}
  </div>
</div>

<div class="light-box">
  <div>
    <i>
      {$t('home.description3')}
    </i>
  </div>
  <div class="centered">
    <a href="./collection">
      <button class="light"> {$t('home.collection-button')} </button>
    </a>
    <a href="./courses">
      <button class="light"> {$t('home.courses-button')} </button>
    </a>
  </div>
</div>

<div class="dark-stripe">
  <div
    class="dancers"
    style="grid-template-columns: repeat({bigNumDancers}, 1fr);"
  >
    {#each { length: bigNumDancers } as _}
      <DanceAnimation
        dance={dance(2)}
        style={BOLD_MAIN_THEME_COLORING}
        showOverflow
      />
    {/each}
  </div>
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
    style="grid-template-columns: repeat({midNumDancers}, 1fr);"
  >
    {#each { length: midNumDancers } as _}
      <DanceAnimation
        dance={dance(3)}
        style={BOLD_MAIN_THEME_COLORING}
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
    /* box-shadow: var(--theme-neutral-white) 0px 0px 11px; */
    padding: 10px 100%;
    margin: -10px -100%;
    background-color: var(--theme-neutral-dark);
    box-shadow: var(--theme-neutral-dark) 0px 0px 11px;
  }
  .light-box {
    padding: 20px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
    margin: 5px 0;
    z-index: 1;
    position: relative;
    box-shadow: var(--theme-neutral-dark) 0px 0px 11px;
  }
  .centered {
    margin-top: 15px;
    text-align: center;
  }
  .slogan {
    box-shadow: var(--theme-neutral-white) 0px 0px 5px;
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
