<script>
  import TrackAudio from '$lib/components/TrackAudio.svelte';
  import { songs } from '$lib/stores/Songs';
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';

  /** @type {Song | undefined} */
  let track = undefined;

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

  setContext('music', { setTrack, songTitle, songAuthor });
</script>

{#if track}
  <TrackAudio {track} />
{/if}

<slot />
