<script>
  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<main>
  <PoseDetectionContext>
    <slot />
  </PoseDetectionContext>
</main>
<TabNavigation />

<style>
  main {
    margin: 5px;
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      margin: auto;
    }
  }
</style>
