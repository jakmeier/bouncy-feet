<script>
  /* A left-right swiping banner view of a dance performance. */

  import { onMount } from 'svelte';
  import BannerStep from './BannerStep.svelte';
  import { base } from '$app/paths';

  const avatarSize = 60;

  /** @type{import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let steps = [];
  /** @type {number} */
  export let reviewStart;
  /** @type {number} */
  export let reviewEnd;

  /**
   * @type {number}
   * Between 0.0 and 1.0, depending on where in the scrollable range the center should be.
   * Note that the scrollable range is typically larger than the range of detected steps.
   */
  let cursor = 0;
  /**
   * Manually called by parent. Due to cyclic reactivity, it seems easier than
   * using reactive statements (but maybe I just don't know how to use them
   * properly in such cases)
   * @param {number} newCursorValue
   */
  export function setCursor(newCursorValue) {
    cursor = newCursorValue;
    adjustScroll(scrollableWidth, cursor);
  }
  /**
   * @type {undefined | ((cursor: number) => void)}
   * called when the parent should be notified about scrolling
   * (the other direction of setCursor)
   */
  export let onScroll = undefined;

  /**
   * @type {HTMLDivElement}
   */
  let stepsDiv;
  let visibleBannerWidth = 0;

  // minimum 500px wide banner, bu it should scale with more drawn avatars
  $: scrollableWidth = Math.max(500, steps.length * avatarSize * 4);
  // scroll position zero should put the first possible pose in the center
  // for this, we have to offset all positions by `scrollOffset`
  $: scrollOffset = visibleBannerWidth / 2 - avatarSize;
  // likewise, cursor=1 should center the last possible pose hence, put a fake
  // element in the banner to reserve extra space in it, here we compute the
  // position of it
  $: innerWidth = scrollableWidth + 2 * scrollOffset;

  /**
   * @param {number} scrollableWidth
   * @param {number} cursor
   */
  async function adjustScroll(scrollableWidth, cursor) {
    // avoid cyclic update
    if (stepsDiv) {
      stepsDiv.scrollLeft = scrollableWidth * cursor;
    }
  }

  function scrolled() {
    const r = stepsDiv.scrollLeft / scrollableWidth;
    if (onScroll) {
      onScroll(r);
    }
    cursor = r;
  }

  onMount(() => {
    adjustScroll(scrollableWidth, cursor);
  });
</script>

<div id="container" bind:clientWidth={visibleBannerWidth}>
  <img class="arrow" src="{base}/img/left_arrow.svg" alt="left arrow" />
  <div id="steps" bind:this={stepsDiv} on:scroll={scrolled}>
    {#each steps as step}
      <BannerStep
        {step}
        {scrollableWidth}
        {scrollOffset}
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
  <img class="invert arrow" src="{base}/img/left_arrow.svg" alt="left arrow" />
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
</style>
