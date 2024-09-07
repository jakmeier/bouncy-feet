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
  let outerWidth;
  $: explanationWidth = outerWidth ? outerWidth / 2 : 200;
</script>

<div class="outer" bind:clientWidth={outerWidth}>
  <div class="explanation">
    <Explanation text={$t(text)} width={explanationWidth}></Explanation>
  </div>

  <!-- TODO: would be nicer to show this visually instead of with numbers -->
  <p>{(hitRate * 100).toFixed(0)}%</p>
  <!-- <p>{hits} / {hits + misses}</p> -->
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
  .explanation {
    margin: 20px;
  }
</style>
