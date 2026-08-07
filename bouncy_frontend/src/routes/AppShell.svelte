<script>
  import { run } from 'svelte/legacy';
  import TabNavigation from './TabNavigation.svelte';
  import { dev } from '$lib/stores/FeatureSelection';
  import {
    fontColor,
    backgroundColor,
    hideNavigation,
    wideView,
    pageTransition,
  } from '$lib/stores/UiState.svelte.js';
  import PoseDetectionContext from './PoseDetectionContext.svelte';
  import UserContext from '../lib/stores/UserContext.svelte';
  import LocalCollectionContext from './LocalCollectionContext.svelte';
  import MusicContext from '../lib/components/audio/MusicContext.svelte';
  import LocalStateContext from './LocalStateContext.svelte';
  import UserAvatarStyleContext from '$lib/components/avatar/UserAvatarStyleContext.svelte';
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';
  import { fade } from 'svelte/transition';
  import PageTransitionOverlay from './PageTransitionOverlay.svelte';
  import ClubsContext from '$lib/stores/ClubsContext.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {any} data
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
    courses.find((/** @type {any} */ course) => course.id === id);
  setContext('courses', { courses: readable(courses), getCourse });
</script>

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
                <ClubsContext publicClubs={data.publicClubs}>
                  {#if pageTransition.kind !== 'coach' || !pageTransition.active || pageTransition.phase === 'settle' || pageTransition.phase === 'finishing'}
                    <div out:fade in:fade={{ delay: 600 }}>
                      {@render children?.()}
                    </div>
                  {/if}
                </ClubsContext>
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

<PageTransitionOverlay />

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
      margin: auto !important;
    }
  }
</style>
