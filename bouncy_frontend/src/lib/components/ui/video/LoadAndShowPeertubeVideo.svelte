<script>
  import * as api from '$lib/peertube-openapi';
  import PeertubeVideoPlayer from './PeertubeVideoPlayer.svelte';
  import VideoLoader from './VideoLoader.svelte';

  /** @type {api.Video | undefined}*/
  let video = $state();

  /**
   * @typedef {Object} Props
   * @prop {number|string} videoId
   * @property {"inline"|"external"} [timeline]
   */

  /** @type {Props}*/
  let { videoId, timeline } = $props();
</script>

{#if !video}
  <VideoLoader {videoId} onLoaded={(v) => (video = v)} />
{:else if video.shortUUID}
  <div class="video">
    <PeertubeVideoPlayer
      videoId={video.shortUUID}
      aspectRatio={video.aspectRatio || 1}
      {timeline}
    />
  </div>
{:else}
  Video missing shortUuid
{/if}
