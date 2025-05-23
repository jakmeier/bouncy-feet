<script>
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { t } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @property {string} teacherName
   * @property {any} step
   * @property {any} skill
   * @property {any} maxSkill
   * @property {any} experience
   * @property {any} totalSteps - export let lastTraining;
   */

  /** @type {Props} */
  let { teacherName, step, skill, maxSkill, experience, totalSteps } = $props();
</script>

<h3>{teacherName}</h3>

<div class="container">
  <div class="coach">
    {#if step}
      <AnimatedStep {step} size={220} backgroundColor="none" />
    {/if}
  </div>

  <div class="skill">
    <div>{$t('pathway.skill')}</div>
    <div>{skill} / {maxSkill}</div>
  </div>

  <div class="experience">
    <div>XP</div>
    <div>{experience}</div>
  </div>

  <div class="stats">
    <div>{$t('stats.num-steps')}</div>
    <div>{totalSteps}</div>
  </div>
</div>

<style>
  .container {
    display: grid;
    width: 100%;
    grid-template-columns: 1fr 1fr 1fr;
    text-align: center;
  }

  .coach {
    grid-column: 1 / 4;
    margin: 0 0 25px;
  }

  .skill,
  .stats {
    display: grid;
    grid-template-columns: 1fr;
  }
</style>
