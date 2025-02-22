<script>
  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import {
    fontColor,
    backgroundColor,
    hideNavigation,
    wideView,
  } from '$lib/stores/UiState.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from './UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';
  import { dev } from '$lib/stores/FeatureSelection';
  import MusicContext from './MusicContext.svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';

  $: hideNavigation.set(!$dev);
  $: navBarHeight = $hideNavigation ? 0 : 90;
  $: outerPadding = $wideView ? '0rem' : '1rem';
  $: mainMargin = $wideView ? 0 : 5;
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<div
  class="background"
  style="--background-color: {$backgroundColor}; --color:{$fontColor}; padding: 0 {outerPadding};"
>
  <main
    style="margin:{mainMargin}px; height: calc(100vh - {navBarHeight}px); max-width: calc(min(730px, 100vw) - 2 * {outerPadding});"
  >
    <UserContext>
      <LocalCollectionContext>
        <PoseDetectionContext>
          <MusicContext>
            <AvatarStyleContext>
              <slot />
            </AvatarStyleContext>
          </MusicContext>
        </PoseDetectionContext>
      </LocalCollectionContext>
    </UserContext>
    {#if !$hideNavigation}
      <div class="scroll-buffer"></div>
    {/if}
  </main>
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
    background-color: var(--background-color);
    color: var(--color);
  }

  @media (min-width: 730px) {
    main {
      max-width: 720px;
      /* !important to overrule the inline style */
      margin: auto !important;
    }
  }
</style>
