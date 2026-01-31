<script>
  import { goto } from '$app/navigation';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { t } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @property {ApiUser} apiUser
   */

  /** @type {Props} */
  let { apiUser } = $props();
</script>

<StandardPage title="" white={false}>
  <h3>{$t('home.continue-first-warmup')}</h3>
  <button onclick={() => goto('firstCourse')}>
    {$t('home.continue-button')}
  </button>
  <button onclick={() => apiUser.setSkippedIntro(true)}>
    {$t('home.skip-button')}
  </button>
</StandardPage>

<style>
  button {
    margin-right: 1rem;
  }
</style>
