<script>
  import ThumbnailFeed from '$lib/components/ThumbnailFeed.svelte';
  import { fetchPlaylist } from '$lib/peertube';
  import Symbol from '../Symbol.svelte';

  /**
   * @typedef {Object} Props
   * @property {PlaylistInfo} playlistInfo
   * @property {boolean} [editable]
   */

  /** @type {Props} */
  let { playlistInfo, editable = false } = $props();

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
    {#if editable}
      <div class="symbol">
        <a href={`./playlist/${playlist.shortUUID}/edit`}>
          <Symbol size={32}>edit</Symbol>
        </a>
      </div>
    {/if}
    <h2>{playlist.displayName}</h2>
    <p>{playlist.description}</p>
    <ThumbnailFeed bind:this={feed} playlistUuid={playlistInfo.short_uuid}
    ></ThumbnailFeed>
  </div>
{/await}

<style>
  .playlist {
    position: relative;
  }

  .symbol {
    position: absolute;
    right: 0;
    top: -1rem;
  }
</style>
