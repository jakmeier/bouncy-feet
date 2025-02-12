<script>
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
</script>

<Header title={$t('editor.title')}></Header>

<div class="centered">
  <p>
    {$t('editor.description0')}
    {$t('editor.description1')}
  </p>
  <i>
    {$t('editor.description2')}
  </i>

  <div class="buttons">
    <a href="./pose">
      <button class="wide"> {$t('editor.pose.list')} </button>
    </a>
    <a href="./step">
      <button class="wide"> {$t('editor.step.list')} </button>
    </a>
  </div>
</div>

<style>
  .centered {
    max-width: 80%;
    margin: auto;
    text-align: center;
  }
  .buttons {
    margin-top: 20px;
  }
</style>
