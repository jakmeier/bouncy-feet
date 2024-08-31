<script>
  import Explanation from '$lib/components/ui/Explanation.svelte';
  import SpeechBubble from '$lib/components/ui/SpeechBubble.svelte';
  import { t } from '$lib/i18n';

  export let hits;
  export let misses;
  export let hitRate;
  export let passed;

  let text =
    hitRate > 0.6
      ? 'courses.end.success'
      : hitRate > 0.44
        ? 'courses.end.failed'
        : 'courses.end.failed-hard';

  function goBack() {
    window.history.back();
    window.history.back();
  }
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

  <p>{hitRate.toPrecision(2) * 100}%</p>
  <p>{hits} / {hits + misses}</p>

  <div class="explanation">
    <Explanation text={$t(text)}></Explanation>
  </div>

  <button class="light" on:click={goBack}
    >{$t('courses.end.back-button')}</button
  >
  <!-- TODO try again button -->
</div>

<style>
  .outer {
    display: flex;
    flex-direction: column;
    margin-top: 100px;
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
