<script>
  import {
    incrementalExperienceForLevel,
    experienceToLevel,
    totalExperienceForLevel,
  } from '$lib/stats';

  // A component to show the level and experience of a step or dance.
  /** @type {number} */
  export let xp;
  /** @type {number} */
  export let height = 20;
  export let lvlSize = 35;
  export let twoRows = false;

  $: level = experienceToLevel(xp);
  $: xpRequired = incrementalExperienceForLevel(level + 1);
  $: xpProgress = xp - totalExperienceForLevel(level);
</script>

<div
  class="outer"
  style="grid-template-columns: {twoRows ? '100%' : '30px auto'};"
>
  <div
    class="level"
    style="width: {lvlSize}px; height: {lvlSize}px; line-height: {lvlSize}px; font-size: {lvlSize/2}px;"
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
    transition: width 0.5s;
  }

  .level {
    background-color: var(--theme-neutral-light);
    border-radius: 50%;
    text-align: center;
  }
</style>
