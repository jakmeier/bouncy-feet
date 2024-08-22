<script>
  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import {
    backgroundColor,
    hideNavigation,
    wideView,
  } from '$lib/stores/UiState.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from './UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';

  $: navBarHeight = $hideNavigation ? 0 : 90;
  $: outerPadding = $wideView ? 0 : 5;
  $: mainMargin = $wideView ? 2 : 5;
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<div style="background-color: {$backgroundColor}; padding: {outerPadding}px;">
  <main
    style="margin:{mainMargin}px; height: calc(100vh - {navBarHeight}px); max-width: calc(min(730px, 100vw) - {2 *
      mainMargin}px);"
  >
    <UserContext>
      <LocalCollectionContext>
        <PoseDetectionContext>
          <slot />
        </PoseDetectionContext>
      </LocalCollectionContext>
    </UserContext>
    <div class="scroll-buffer"></div>
  </main>
</div>

{#if !$hideNavigation}
  <TabNavigation height={navBarHeight} />
{/if}

<style>
  .scroll-buffer {
    height: 100px;
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      /* !important to overrule the inline style */
      margin: auto !important;
    }
  }
</style>
