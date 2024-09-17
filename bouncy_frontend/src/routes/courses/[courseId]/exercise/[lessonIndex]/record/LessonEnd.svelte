<script>
  import Explanation from '$lib/components/ui/Explanation.svelte';
  import { t } from '$lib/i18n';
  import { dances } from '$lib/instructor/bouncy_instructor';
  import { onMount, afterUpdate } from 'svelte';

  /** @type {number} */
  export let hitRate;
  /** @type {boolean} */
  export let passed;

  $: text =
    hitRate >= 0.6
      ? 'courses.end.success'
      : hitRate > 0.44
        ? 'courses.end.failed'
        : 'courses.end.failed-hard';
  let borderColor = 'var(--theme-neutral-gray)';
  let borderWidth = '3px';

  let outerWidth;
  $: explanationWidth = outerWidth ? outerWidth / 2 : 200;
  let scoreWidth = 0;

  const celebrationDance = passed
    ? dances().find((dance) => dance.id === 'Celebrate')
    : null;

  onMount(() => {
    // set this after the initial render to trigger the animation
    setResult();
  });
  afterUpdate(() => {
    setResult();
  });

  function setResult() {
    scoreWidth = hitRate * 100;
    if (passed) {
      borderColor = '#33a86d';
      borderWidth = '5px';
    } else {
      borderColor = '#eb3b3b';
    }
  }
</script>

<div class="outer" bind:clientWidth={outerWidth}>
  <div class="explanation">
    <Explanation
      text={$t(text)}
      width={explanationWidth}
      entryDance={celebrationDance}
    ></Explanation>
  </div>

  <div
    class="result"
    style="border-color: {borderColor}; border-width: {borderWidth}"
  >
    <div class="score" style="width: {scoreWidth}%;"></div>
    {#if passed}
      <span class="material-symbols-outlined above-bar" translate="no">
        verified
      </span>
    {:else}
      <span class="material-symbols-outlined above-bar" translate="no">
        sentiment_dissatisfied
      </span>
    {/if}
    <!-- <span class="material-symbols-outlined above-bar" translate="no"> star_half </span> -->
    <div class="above-bar">
      {(hitRate * 100).toFixed(0)}%
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
    font-size: 72px;
    margin: 5px;
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
    background-color: var(--theme-main);
    height: 100%;
    top: 0;
    left: 0;
    width: 0;
    transition: width 1s;
  }
  span {
    font-size: 36px;
    margin: 10px;
  }
  .explanation {
    margin: 20px;
  }
</style>
