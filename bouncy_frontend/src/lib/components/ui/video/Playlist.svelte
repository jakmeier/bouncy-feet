<script>
  import ThumbnailFeed from '$lib/components/ThumbnailFeed.svelte';
  import { fetchPlaylist } from '$lib/peertube';

  /**
   * @typedef {Object} Props
   * @property {PlaylistInfo} playlistInfo
   */

  /** @type {Props} */
  let { playlistInfo } = $props();

  const playlistPromise = fetchPlaylist(playlistInfo.short_uuid);
</script>

{#await playlistPromise}
  <p>Loading playlist...</p>
{:then playlist}
  <h2>{playlist.displayName}</h2>
  <p>{playlist.description}</p>
  <ThumbnailFeed playlistUuid={playlistInfo.short_uuid}></ThumbnailFeed>
{/await}
