<script>
  import { t } from '$lib/i18n.js';
  import { onDestroy, onMount } from 'svelte';
  import DanceAnimation from './DanceAnimation.svelte';
  import { backgroundColor } from '$lib/stores/UiState';

  /** @type{null|import("$lib/instructor/bouncy_instructor").DanceInfo} */
  export let featuredDance;

  const numDancers = 5;

  let swapBackgroundColor = 'var(--theme-neutral-white)';
  onMount(() => {
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-main)';
  });
  onDestroy(() => {
    $backgroundColor = swapBackgroundColor;
  });
</script>

<h1>{$t('home.title')}</h1>

<div class="light-box">{$t('home.description0')}</div>

<div class="dancers" style="grid-template-columns: repeat({numDancers}, 1fr);">
  {#if featuredDance}
    {#each { length: numDancers } as _}
      <DanceAnimation
        dance={featuredDance}
        leftColor={'var(--theme-accent-light)'}
        rightColor={'var(--theme-accent-light)'}
        headColor={'var(--theme-accent-light)'}
        bodyColor={'var(--theme-accent-light)'}
      />
    {/each}
  {/if}
</div>

<div class="light-box">
  <div>
    {$t('home.description2')}
  </div>
  <div class="centered">
    <a href="https://github.com/jakmeier/bouncy-feet/issues">
      <button class="light"> Issue Tracker </button>
    </a>
  </div>
</div>

<style>
  h1 {
    margin: -5px -5px 15px -5px;
    background-color: var(--theme-main);
    color: var(--theme-accent);
    width: 100vw;
    padding: 25px 0;
    font-weight: 900;
    text-align: center;
  }
  .dancers {
    display: grid;
    margin: 20px;
  }
  .light-box {
    padding: 20px;
    background-color: var(--theme-neutral-light);
    border-radius: 10px;
  }
  .centered {
    margin-top: 15px;
    text-align: center;
  }
</style>
