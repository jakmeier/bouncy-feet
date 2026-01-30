<script>
  import { goto } from '$app/navigation';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { getUserContext } from '$lib/stores/context';
  import { t } from '$lib/i18n';

  /** @type {UserContextData}*/
  const userCtx = getUserContext();
</script>

<StandardPage title="" white={false}>
  <h3>{$t('home.continue-first-warmup')}</h3>
  <button onclick={() => goto('firstCourse')}>
    {$t('home.continue-button')}
  </button>
  <button onclick={() => userCtx.apiUser.setSkippedIntro(true)}>
    {$t('home.skip-button')}
  </button>
</StandardPage>

<style>
  button {
    margin-right: 1rem;
  }
</style>
