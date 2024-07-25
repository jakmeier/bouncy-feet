<script>
  import {
    incrementalExperienceForLevel,
    experienceToLevel,
    totalExperienceForLevel,
  } from '$lib/stats';

  // A component to show the level and experience of a step or dance.
  /** @type {number} */
  export let xp;

  $: level = experienceToLevel(xp);
  $: xpRequired = incrementalExperienceForLevel(level + 1);
  $: xpProgress = xp - totalExperienceForLevel(level);
</script>

<div class="outer">
  <div class="level">{level}</div>
  <div class="progress-container">
    <div
      class="progress-bar"
      style="width: {(100 * xpProgress) / xpRequired}%;"
    ></div>
  </div>
</div>

<style>
  .outer {
    display: grid;
    grid-template-columns: 30px auto;
    align-items: center;
    margin: -10px 10px 0 10px;
  }
  .progress-container {
    width: 100%;
    height: 20px;
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    overflow: hidden;
  }

  .progress-bar {
    height: 15px;
    margin: 2.5px;
    background-color: var(--theme-main);
    border-radius: 5px;
    width: 0;
    transition: width 0.5s;
  }

  .level {
    width: 35px;
    height: 35px;
    line-height: 35px;
    font-size: 1.2em;
    background-color: var(--theme-neutral-light);
    border-radius: 50%;
  }
</style>
