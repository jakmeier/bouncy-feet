<script>
  import { t } from '$lib/i18n.js';
  import { steps } from '$lib/instructor/bouncy_instructor';
  import Step from './Step.svelte';

  let stepList = steps();
</script>

<h1>{$t('home.title')}</h1>
<p>[intro video]</p>
<p>[recommendations]</p>

<h2>{$t('home.steps')}</h2>
<div class="step-table">
  {#each stepList as step}
    <Step {step} />
  {/each}
</div>

<style>
  .step-table {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    text-align: center;
    gap: 20px 0px;
  }
</style>
