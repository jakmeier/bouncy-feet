<script>
  import { goto } from '$app/navigation';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { t } from '$lib/i18n';
</script>

<StandardPage title="" white={false}>
  <h3>{$t('home.continue-first-warmup')}</h3>
  <button onclick={() => goto('firstWarmup')}>
    {$t('home.continue-button')}
  </button>
</StandardPage>
