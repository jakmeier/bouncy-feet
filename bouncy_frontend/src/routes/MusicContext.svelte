<script>
  import TrackAudio from '$lib/components/TrackAudio.svelte';
  import { songs } from '$lib/stores/Songs';
  import { setContext } from 'svelte';

  /** @type {Song | undefined} */
  let track = undefined;

  /** @param {string} newTrackId */
  async function setTrack(newTrackId) {
    track = songs.get(newTrackId);
  }

  setContext('music', { setTrack });
</script>

{#if track}
  <TrackAudio {track} />
{/if}

<slot />
