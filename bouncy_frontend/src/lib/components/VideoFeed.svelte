<script>
  import { fetchVideosOfPlaylist } from '$lib/peertube';
  import Juggler from './ui/Juggler.svelte';

  let { playlistId } = $props();
  let videoIds = $derived(fetchVideoIds());

  async function fetchVideoIds() {
    const videos = await fetchVideosOfPlaylist(playlistId);
    return videos.data.map((v) => v.video.uuid);
  }
</script>

<div class="outer">
  <!-- TODO(August): like video -->
  <!-- TODO(August): report video -->

  {#await videoIds then ids}
    <Juggler {ids}></Juggler>
  {/await}
</div>

<style>
  .outer {
    margin: auto;
    width: 90%;
    height: 90%;
  }
</style>
