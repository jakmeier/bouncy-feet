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
    $backgroundColor = 'var(--theme-neutral-white)';
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
        leftColor={'var(--theme-main)'}
        rightColor={'var(--theme-main)'}
        headColor={'var(--theme-main)'}
        bodyColor={'var(--theme-main)'}
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
    padding: 25px 0;
    text-align: center;
    text-shadow: var(--theme-main) 0px 0px 11px;
    font-size: 45px;
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
