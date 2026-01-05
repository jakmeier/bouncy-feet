<script>
  import ThumbnailFeed from '$lib/components/ThumbnailFeed.svelte';
  import { fetchPlaylist } from '$lib/peertube';

  /**
   * @typedef {Object} Props
   * @property {PlaylistInfo} playlistInfo
   */

  /** @type {Props} */
  let { playlistInfo } = $props();

  let feed = $state();

  const playlistPromise = fetchPlaylist(playlistInfo.short_uuid);

  export async function refreshVideos() {
    feed.refreshVideos();
  }
</script>

{#await playlistPromise}
  <p>Loading playlist...</p>
{:then playlist}
  <div class="playlist">
    <h2>{playlist.displayName}</h2>
    <p>{playlist.description}</p>
    <ThumbnailFeed bind:this={feed} playlistUuid={playlistInfo.short_uuid}
    ></ThumbnailFeed>
  </div>
{/await}

<style>
  .playlist {
    background-color: var(--theme-main);
  }
</style>
