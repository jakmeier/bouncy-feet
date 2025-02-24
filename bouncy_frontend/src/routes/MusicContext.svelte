<script>
  import TrackAudio from '$lib/components/TrackAudio.svelte';
  import { songs } from '$lib/stores/Songs';
  import { onMount, setContext } from 'svelte';
  import { readable } from 'svelte/store';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /** @type {Song | undefined} */
  let track = $state(undefined);

  let titleSetter = (_) => {};
  const songTitle = readable('-', (set) => {
    titleSetter = set;
  });

  let authorSetter = (_) => {};
  const songAuthor = readable('-', (set) => {
    authorSetter = set;
  });

  /** @param {string} newTrackId */
  async function setTrack(newTrackId) {
    track = songs.get(newTrackId);
    titleSetter(track?.title || '-');
    authorSetter(track?.author || '-');
  }

  let resetTrackInternal = $state(() => {});
  function resetTrack() {
    resetTrackInternal();
  }

  let stopTrackInternal = $state(() => {});
  function stopTrack() {
    stopTrackInternal();
  }

  setContext('music', {
    setTrack,
    resetTrack,
    stopTrack,
    songTitle,
    songAuthor,
  });
</script>

{#if track}
  <TrackAudio
    {track}
    bind:resetTrack={resetTrackInternal}
    bind:stopMusic={stopTrackInternal}
  />
{/if}

{@render children?.()}
