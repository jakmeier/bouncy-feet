<script>
  import Explanation from '$lib/components/ui/Explanation.svelte';
  import { t } from '$lib/i18n';

  export let hits;
  export let misses;
  export let hitRate;
  export let passed;

  $: text =
    hitRate >= 0.6
      ? 'courses.end.success'
      : hitRate > 0.44
        ? 'courses.end.failed'
        : 'courses.end.failed-hard';
</script>

<div class="outer">
  {#if passed}
    <span class="material-symbols-outlined done"> verified </span>
  {:else if hitRate > 0.44}
    <span class="material-symbols-outlined done"> sports_score </span>
  {:else}
    <p>
      <span class="material-symbols-outlined done">
        sentiment_dissatisfied
      </span>
    </p>
  {/if}

  <p>{(hitRate * 100).toFixed(0)}%</p>
  <p>{hits} / {hits + misses}</p>

  <div class="explanation">
    <Explanation text={$t(text)}></Explanation>
  </div>
</div>

<style>
  .outer {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
  }
  p {
    font-size: 36px;
    margin: 5px;
  }
  span {
    font-size: 130px;
    color: var(--theme-neutral-dark);
  }
  .explanation {
    margin: 20px;
  }
</style>
