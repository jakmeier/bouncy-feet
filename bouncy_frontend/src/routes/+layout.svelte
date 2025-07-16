<script>
  import { run } from 'svelte/legacy';

  import TabNavigation from './TabNavigation.svelte';
  import '../app.css';
  import { t } from '$lib/i18n.js';
  import {
    fontColor,
    backgroundColor,
    hideNavigation,
    wideView,
    fadingOut,
    floatingTitleRect,
  } from '$lib/stores/UiState.svelte.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from './UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';
  import { dev } from '$lib/stores/FeatureSelection';
  import MusicContext from '../lib/components/audio/MusicContext.svelte';
  import LocalStateContext from './LocalStateContext.svelte';
  import UserAvatarStyleContext from '$lib/components/avatar/UserAvatarStyleContext.svelte';
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';
  import { fade } from 'svelte/transition';
  import { sendPersonalityTitle } from '$lib/stores/Crossfade.svelte';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children, data } = $props();

  run(() => {
    hideNavigation.set(!$dev);
  });
  let navBarHeight = $derived($hideNavigation ? 0 : 90);
  let outerPadding = $derived($wideView ? '0rem' : '1.5rem');

  const courses = data.courses;
  const getCourse = (/** @type {string} */ id) =>
    courses.find((course) => course.id === id);
  setContext('courses', { courses: readable(courses), getCourse });
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

<div
  class="background"
  style="--background-color: {$backgroundColor}; --color:{$fontColor}; padding: 0 {outerPadding};"
>
  <main
    style="height: calc(100vh - {navBarHeight}px); max-width: calc(min(730px, 100vw) - 2 * {outerPadding});"
  >
    <LocalStateContext>
      <UserContext>
        <LocalCollectionContext>
          <PoseDetectionContext>
            <MusicContext>
              <UserAvatarStyleContext>
                {#if !fadingOut.state}
                  <div out:fade in:fade={{ delay: 600 }}>
                    {@render children?.()}
                  </div>
                {/if}
                {#if fadingOut.text}
                  <h1
                    class="floating-title"
                    out:sendPersonalityTitle={{ key: 'pageTitle' }}
                    style="
                      position: fixed;
                      top: {floatingTitleRect.top}px;
                      left: {floatingTitleRect.left}px;
                      color: {fadingOut.color};
                      font-size: {fadingOut.fontSize.current}px;
                      margin: 0;
                      z-index: 999;
                    "
                  >
                    {fadingOut.text}
                  </h1>
                {/if}
              </UserAvatarStyleContext>
            </MusicContext>
          </PoseDetectionContext>
        </LocalCollectionContext>
      </UserContext>
    </LocalStateContext>
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
    width: calc(100% - 3rem);
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
