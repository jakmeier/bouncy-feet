<script>
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import { getContext } from 'svelte';

  const localCollectionCtx = getContext('localCollection');
  const steps = localCollectionCtx.steps;
</script>

<Header title={$t('editor.step.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.step.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.step.list')}</h2>

<div class="steps">
  {#each $steps as step}
    <div class="step">
      <p>{step.name('en')}</p>
    </div>
  {/each}
</div>

<style>
  .steps {
    display: flex;
    flex-wrap: wrap;
  }
  .step {
    max-width: 200px;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }
</style>
