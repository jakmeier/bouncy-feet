<script>
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import ThumbnailJuggler from './ui/ThumbnailJuggler.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} playlistId
   */

  /** @type {Props} */
  let { playlistId } = $props();

  async function fetchVideos() {
    const videos = await fetchVideosOfPlaylist(playlistId);
    return videos.data?.flatMap((v) => (v.video ? [v.video] : []));
  }
</script>

<div class="outer">
  {#await fetchVideos() then videos}
    {#if videos && videos.length > 0}
      <ThumbnailJuggler {videos}></ThumbnailJuggler>
    {:else}
      {$t('video.empty-playlist')}
    {/if}
  {/await}
</div>

<style>
  .outer {
    margin: auto;
    width: 90%;
    height: 90%;
  }
</style>
