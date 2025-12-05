<script>
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import VideoJuggler from './ui/VideoJuggler.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} playlistId
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { playlistId, autoplay = false } = $props();
  let videoUuids = $derived(fetchVideoUuids());

  async function fetchVideoUuids() {
    const videos = await fetchVideosOfPlaylist(playlistId);
    return videos.data?.map((v) => v.video?.uuid);
  }
</script>

<div class="outer">
  <!-- TODO(August): like video -->
  <!-- TODO(August): report video -->

  {#await videoUuids then ids}
    {#if ids?.length > 0}
      <VideoJuggler {ids} {autoplay}></VideoJuggler>
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
