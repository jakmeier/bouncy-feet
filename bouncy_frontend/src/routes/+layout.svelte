<script>
  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from './UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';

  const navBarHeight = 90;
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<main style="height: calc(100vh - {navBarHeight}px)">
  <UserContext>
    <LocalCollectionContext>
      <PoseDetectionContext>
        <slot />
      </PoseDetectionContext>
    </LocalCollectionContext>
  </UserContext>
</main>
<TabNavigation height={navBarHeight} />

<style>
  main {
    margin: 5px;
    max-width: calc(min(730px, 100vw) - 10px);
    overflow-y: auto;
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      margin: auto;
    }
  }
</style>
