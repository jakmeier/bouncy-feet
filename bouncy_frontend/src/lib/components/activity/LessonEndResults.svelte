<script>
  import { t } from '$lib/i18n';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {number} hitRate
   * @property {boolean} passed
   */

  let { hitRate, passed } = $props();

  let text = $derived(
    hitRate >= 0.6
      ? 'courses.end.success'
      : hitRate > 0.44
        ? 'courses.end.failed'
        : 'courses.end.failed-hard'
  );
  let displayedPercent = $derived((Number(hitRate || 0) * 100).toFixed(0));
  let scoreColor = $state('var(--theme-neutral-gray)');
  let borderWidth = '3px';

  let outerWidth = $state();
  let scoreWidth = $state(0);

  onMount(() => {
    // set this after the initial render to trigger the animation
    setResult();
  });

  function setResult() {
    scoreWidth = hitRate * 100;
    if (passed) {
      scoreColor = 'var(--theme-main-alt)';
      // borderWidth = '5px';
    } else {
      scoreColor = '#eb3b3b';
    }
  }
</script>

<div
  class="outer"
  bind:clientWidth={outerWidth}
  style="--score-color: {scoreColor}"
>
  <h1>{$t(text)}</h1>

  <div
    class="result"
    style="border-color: {scoreColor}; border-width: {borderWidth}"
  >
    <div class="score" style="width: {scoreWidth}%;"></div>
    <!-- <span class="material-symbols-outlined above-bar" translate="no"> star_half </span> -->
    <div class="above-bar">
      {displayedPercent}%
    </div>
  </div>
</div>

<style>
  .outer {
    display: flex;
    flex-direction: column;
    margin-top: 10px;
  }
  .result {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-large);
    margin: 2rem;
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    /* width and color overwritten by inline style */
    border: solid 3px var(--theme-neutral-gray);
    border-radius: 38px;
    overflow: hidden;
    transition:
      border-color 0s step-start 1s,
      border-width 0s step-start 1s;
  }
  .above-bar {
    position: relative;
    z-index: 1;
  }
  .score {
    position: absolute;
    background-color: var(--score-color);
    height: calc(100% + 2px);
    margin: -1px;
    top: 0;
    left: 0;
    width: 0;
    transition:
      width 1s,
      background-color 0s step-start 1s;
  }
  h1 {
    color: var(--theme-main);
  }
</style>
