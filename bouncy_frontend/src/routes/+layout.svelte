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
  import MovingBackground from '$lib/components/ui/MovingBackground.svelte';
  import { dev } from '$lib/stores/FeatureSelection';
  import MusicContext from './MusicContext.svelte';

  $: hideNavigation.set(!$dev);
  $: navBarHeight = $hideNavigation ? 0 : 90;
  $: outerPadding = $wideView ? 0 : 5;
  $: mainMargin = $wideView ? 2 : 5;
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<div
  class="background"
  style="background: {$backgroundColor}; padding: {outerPadding}px;"
>
  <MovingBackground>
    <main
      style="margin:{mainMargin}px; height: calc(100vh - {navBarHeight}px); max-width: calc(min(730px, 100vw) - {2 *
        mainMargin}px);"
    >
      <UserContext>
        <LocalCollectionContext>
          <PoseDetectionContext>
            <MusicContext>
              <slot />
            </MusicContext>
          </PoseDetectionContext>
        </LocalCollectionContext>
      </UserContext>
      <div class="scroll-buffer"></div>
    </main>
  </MovingBackground>
</div>

{#if !$hideNavigation}
  <TabNavigation height={navBarHeight} />
{/if}

<style>
  .scroll-buffer {
    height: 100px;
  }

  .background {
    position: fixed;
    z-index: -2;
    overflow: hidden auto;
    width: 100%;
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      /* !important to overrule the inline style */
      margin: auto !important;
    }
  }
</style>
