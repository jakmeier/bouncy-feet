<script>
  import {
    incrementalExperienceForLevel,
    experienceToLevel,
    totalExperienceForLevel,
  } from '$lib/stats';

  // A component to show the level and experience of a step or dance.

  /**
   * @typedef {Object} Props
   * @property {number} xp
   * @property {number} [height]
   * @property {number} [lvlSize]
   * @property {boolean} [twoRows]
   */

  /** @type {Props} */
  let { xp, height = 20, lvlSize = 35, twoRows = false } = $props();

  let level = $derived(experienceToLevel(xp));
  let xpRequired = $derived(incrementalExperienceForLevel(level + 1));
  let xpProgress = $derived(xp - totalExperienceForLevel(level));
</script>

<div
  class="outer"
  style="grid-template-columns: {twoRows ? '100%' : '30px auto'};"
>
  <div
    class="level"
    style="width: {lvlSize}px; height: {lvlSize}px; line-height: {lvlSize}px; font-size: {lvlSize /
      2}px;"
  >
    {level}
  </div>
  <div class="progress-container" style="height: {height}px">
    <div
      class="progress-bar"
      style="width: {(100 * xpProgress) / xpRequired}%; height: {height - 5}px"
    ></div>
  </div>
</div>

<style>
  .outer {
    display: grid;
    align-items: center;
    justify-items: center;
    margin: -10px 10px 0 10px;
  }
  .progress-container {
    width: 100%;
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    overflow: hidden;
  }

  .progress-bar {
    margin: 2.5px;
    background-color: var(--theme-main);
    border-radius: 5px;
    width: 0;
    transition: width 1s;
  }

  .level {
    background-color: var(--theme-neutral-light);
    border-radius: 50%;
    text-align: center;
  }
</style>
