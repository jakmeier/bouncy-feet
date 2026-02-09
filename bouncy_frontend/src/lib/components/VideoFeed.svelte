<script>
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import VideoJuggler from './ui/VideoJuggler.svelte';
  import * as api from '$lib/peertube-openapi';

  /**
   * @typedef {Object} Props
   * @property {number} playlistId
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { playlistId, autoplay = false } = $props();
  /** @type {Promise<(api.Video | undefined)[] | undefined>}  */
  let videosPromise = $derived(fetchVideos());

  /** @returns {Promise<(api.Video | undefined)[] | undefined>}  */
  async function fetchVideos() {
    const videos = await fetchVideosOfPlaylist(playlistId);
    return videos.data?.map((v) => v.video);
  }
</script>

<div class="outer">
  <!-- TODO(publish): like video -->
  <!-- TODO(publish): report video -->

  {#await videosPromise then videos}
    {#if videos && videos?.length > 0}
      <VideoJuggler {videos} {autoplay}></VideoJuggler>
    {:else}
      {$t('video.empty-playlist')}
    {/if}
  {/await}
</div>

<style>
  .outer {
    margin: 0 auto;
    width: calc(100% - 1rem);
  }
</style>
