<script>
  import Juggler from './ui/Juggler.svelte';
  import PeertubeVideoPlayer from './ui/video/PeertubeVideoPlayer.svelte';

  let { playlistId } = $props();
</script>

<div class="outer">
  <!-- TODO(August): like video -->
  <!-- TODO(August): report video -->

  <!-- TODO(July): load video feed, circle through playlist -->
  <Juggler ids={['cKMZN3TBmN6ry2aMZUTbbw', 'sGFG13LpFYV6TZFhrEupSw']}>
    {#snippet items(id)}
      <PeertubeVideoPlayer videoId={id} />
    {/snippet}
  </Juggler>
</div>

<style>
  .outer {
    margin: auto;
    width: 90%;
    height: 90%;
  }
</style>
