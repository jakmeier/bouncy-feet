<script>
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import { onMount } from 'svelte';
  import ThumbnailJuggler from './ui/ThumbnailJuggler.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} playlistUuid
   */

  /** @type {Props} */
  let { playlistUuid } = $props();

  /** @returns {Promise<import('$lib/peertube-openapi').Video[] | undefined>} */
  async function fetchVideos() {
    const videos = await fetchVideosOfPlaylist(playlistUuid);
    return videos.data?.flatMap((v) => (v.video ? [v.video] : []));
  }
  /** @type {import('$lib/peertube-openapi').Video[] | undefined} */
  let videos = $state([]);

  onMount(async () => {
    videos = await fetchVideos();
  });

  export async function refreshVideos() {
    videos = await fetchVideos();
  }
</script>

<div class="outer">
  {#if videos}
    {#if videos.length > 0}
      <ThumbnailJuggler {videos}></ThumbnailJuggler>
    {:else}
      {$t('video.empty-playlist')}
    {/if}
  {/if}
</div>

<style>
  .outer {
    margin: auto;
    height: 240px;
    width: min(280px, calc(100vw - 3rem)); /* PeerTube thumbnail width */
  }
</style>
