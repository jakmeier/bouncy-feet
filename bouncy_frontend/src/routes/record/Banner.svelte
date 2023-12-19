<script>
  import { onMount } from 'svelte';
  /* A left-right swiping banner view of a dance performance. */

  import BannerStep from './BannerStep.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let steps = [];
  export let timestamp = 0;
  export let reviewStart;
  export let reviewEnd;
  const avatarSize = 60;
  const scrollBuffer = 1000;

  $: scrollableWidth = Math.max(500, steps.length * avatarSize * 4);
  $: innerWidth = scrollableWidth + 2 * scrollBuffer;

  let scrollFired = false;
  /**
   * @type {HTMLDivElement}
   */
  let stepsDiv;
  /**
   * @param {number} t
   */
  function updateScrollPosition(t) {
    if (stepsDiv && !scrollFired) {
      stepsDiv.scrollTo(
        scrollBuffer + (t / (reviewEnd - reviewStart)) * scrollableWidth,
        0
      );
    } else {
      scrollFired = false;
    }
  }

  function onScroll() {
    scrollFired = true;
    const r = Math.min(
      Math.max(stepsDiv.scrollLeft - scrollBuffer, 0) / scrollableWidth,
      1.0
    );
    timestamp = reviewStart + r * (reviewEnd - reviewStart);
    console.log(`scroll to ${timestamp}`);
  }

  onMount(() => {
    stepsDiv.scrollTo(scrollBuffer, 0);
  });
  $: updateScrollPosition(timestamp);
</script>

<div id="container">
  <img class="arrow" src="img/left_arrow.svg" alt="left arrow" />
  <div id="steps" bind:this={stepsDiv} on:scroll={onScroll}>
    {#each steps as step}
      <BannerStep
        {step}
        {scrollableWidth}
        scrollOffset={scrollBuffer}
        {reviewStart}
        {reviewEnd}
        {avatarSize}
      />
    {/each}
    <div
      style="position: absolute; left: {innerWidth}px; width:1px; height:1px;"
    ></div>
  </div>
  <div id="marker"></div>
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

  #marker {
    position: absolute;
    border: 5px solid var(--theme-neutral-dark);
    height: 60px;
    width: 60px;
    left: 0;
    right: 0;
    margin: -4px auto;
    pointer-events: none;
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
