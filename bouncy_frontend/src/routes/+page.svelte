<script>
  import { t } from '$lib/i18n.js';
  import { base } from '$app/paths';
  import DanceAnimation from './DanceAnimation.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  let featuredDance = data.allDances.length > 0 ? data.allDances[0] : null;
  const numDancers = 3;
</script>

<h1>{$t('home.title')}</h1>

<p>{$t('home.description0')}</p>
<p>{$t('home.description1')}</p>
<p>
  {$t('home.description2')}
  <a href="https://github.com/jakmeier/bouncy-feet/issues">
    GitHub Issue Tracker
  </a>
</p>

<div style="text-align: center;">
  <a href="{base}/record">
    <button>
      <span class="material-symbols-outlined"> directions_walk </span>
      <p>{$t('home.go-button')}</p>
    </button>
  </a>
</div>

<div class="dancers" style="grid-template-columns: repeat({numDancers}, 1fr);">
  {#if featuredDance}
    {#each { length: numDancers } as _}
      <DanceAnimation dance={featuredDance} />
    {/each}
  {/if}
</div>

<style>
  button {
    width: min(50vw, 250px);
    height: min(50vw, 200px);
  }
  button span {
    font-size: min(20vw, 100px);
  }
  button p {
    font-size: 20px;
  }
  .dancers {
    display: grid;
  }
</style>
