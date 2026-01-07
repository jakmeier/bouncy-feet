<script>
  import * as api from '$lib/peertube-openapi';
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import { onMount } from 'svelte';
  import ThumbnailJuggler from './ui/ThumbnailJuggler.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} playlistUuid
   * @property {boolean} [userExtraInfo]
   * @property {boolean} [clubExtraInfo]
   * @property {string} [height]
   * @property {(video: api.Video, index: number)=>void} [onDelete]
   */

  /** @type {Props} */
  let {
    playlistUuid,
    userExtraInfo,
    clubExtraInfo,
    height = '240px',
    onDelete,
  } = $props();

  /** @type {api.Video[] | undefined} */
  let videos = $state([]);
  /** @type {(number | undefined)[] | undefined} */
  let videoPositions = $state([]);

  /** @returns {Promise<import('$lib/peertube-openapi').Video[] | undefined>} */
  async function fetchVideos() {
    const videos = await fetchVideosOfPlaylist(playlistUuid);
    videoPositions = videos.data?.flatMap((v) => (v.video ? [v.position] : []));
    return videos.data?.flatMap((v) => (v.video ? [v.video] : []));
  }
  /** @type {import('$lib/peertube-openapi').Video[] | undefined} */

  onMount(async () => {
    videos = await fetchVideos();
  });

  export async function refreshVideos() {
    videos = await fetchVideos();
  }
</script>

<div class="outer" style:height>
  {#if videos}
    {#if videos.length > 0}
      <ThumbnailJuggler {videos} {userExtraInfo} {clubExtraInfo} {onDelete}
      ></ThumbnailJuggler>
    {:else}
      {$t('video.empty-playlist')}
    {/if}
  {/if}
</div>

<style>
  .outer {
    margin: auto;
    width: min(280px, calc(100vw - 3rem)); /* PeerTube thumbnail width */
  }
</style>
