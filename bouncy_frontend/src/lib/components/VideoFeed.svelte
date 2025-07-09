<script>
  import PeertubePlaylistPlayer from './ui/video/PeertubePlaylistPlayer.svelte';
  import PeertubeVideoPlayer from './ui/video/PeertubeVideoPlayer.svelte';
</script>

<div class="outer">
  <!-- TODO(July): load video feed, circle through playlist -->
  <!-- TODO(July): like video -->
  <!-- TODO(July): report video -->
  <PeertubePlaylistPlayer playlistId="3FWFPJds8qwVJUbzSsjFzH" />
</div>

<style>
  .outer {
    margin: auto;
  }
</style>
