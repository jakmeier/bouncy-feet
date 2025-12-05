<script>
  import { t } from '$lib/i18n';
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import Juggler from './ui/Juggler.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} playlistId
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { playlistId, autoplay = false } = $props();
  let videoIds = $derived(fetchVideoIds());

  async function fetchVideoIds() {
    const videos = await fetchVideosOfPlaylist(playlistId);
    return videos.data?.map((v) => v.video?.uuid);
  }
</script>

<div class="outer">
  <!-- TODO(August): like video -->
  <!-- TODO(August): report video -->

  {#await videoIds then ids}
    {#if ids?.length > 0}
      <Juggler {ids} {autoplay}></Juggler>
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
