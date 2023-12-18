<script>
  /* A left-right swiping banner view of a dance performance. */

  import BannerStep from './BannerStep.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let steps = [];
  const avatarSize = 60;

  $: reviewStart = steps.length > 0 ? steps[0].start : 0;
  $: reviewEnd = steps.length > 0 ? steps[steps.length - 1].end : 1;
  $: width = Math.max(500, steps.length * avatarSize * 4);
</script>

<div id="container">
  <img class="arrow" src="img/left_arrow.svg" alt="left arrow" />
  <div id="steps">
    {#each steps as step}
      <BannerStep
        {step}
        totalWidth={width}
        {reviewStart}
        {reviewEnd}
        {avatarSize}
      />
    {/each}
  </div>
  <img class="invert arrow" src="img/left_arrow.svg" alt="left arrow" />
</div>

<style>
  #container {
    width: 100%;
    display: grid;
    grid-template-columns: 20px 1fr 20px;
    gap: 5px;
    text-align: center;
    margin: 5px -25px;
  }

  #steps {
    position: relative;
    overflow: scroll;
    height: 80px;
    width: 100%;
  }

  .arrow {
    height: 60px;
    width: 15px;
  }

  .invert {
    transform: scaleX(-1);
  }

  @media (max-width: 731px) {
    #container {
      grid-template-columns: 5px 1fr 5px;
      margin: 2px 0;
      gap: 10px;
    }
    .arrow {
      width: 5px;
      padding: 0 2.5px;
      background-color: var(--theme-neutral-light);
    }
  }
</style>
