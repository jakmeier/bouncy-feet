<script>
  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import { backgroundColor, hideNavigation } from '$lib/stores/UiState.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from './UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';

  $: navBarHeight = $hideNavigation ? 0 : 90;
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<div style="background-color: {$backgroundColor}; padding: 5px;">
  <main style="height: calc(100vh - {navBarHeight}px)">
    <UserContext>
      <LocalCollectionContext>
        <PoseDetectionContext>
          <slot />
        </PoseDetectionContext>
      </LocalCollectionContext>
    </UserContext>
    <div class=scroll-buffer></div>
  </main>
</div>

{#if !$hideNavigation}
  <TabNavigation height={navBarHeight} />
{/if}

<style>
  main {
    margin: 5px;
    max-width: calc(min(730px, 100vw) - 10px);
  }

  .scroll-buffer {
    height: 100px;
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      margin: auto;
    }
  }
</style>
