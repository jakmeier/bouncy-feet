<script>
  import * as api from '$lib/peertube-openapi';
  import PeertubeVideoPlayer from './PeertubeVideoPlayer.svelte';
  import VideoLoader from './VideoLoader.svelte';
  import VideoMarkerLoader from './VideoMarkerLoader.svelte';

  /** @type {api.Video | undefined}*/
  let video = $state();
  /** @type {PeertubeVideoPlayer | undefined}*/
  let player = $state();
  /** @type {VideoMarker[] | undefined} */
  let markers = $state();
  /** @type {number[] | undefined} */
  let beats = $state();

  /**
   * @typedef {Object} Props
   * @prop {number|string} videoId
   * @property {"inline"|"external"} [timeline]
   * @property {number} [comboId]
   * @property {ApiUser} [apiUser]
   */

  /** @type {Props}*/
  let { videoId, timeline, comboId, apiUser } = $props();

  /** @returns {Promise<number>} seconds */
  export async function getCurrentTime() {
    if (player) {
      return player.getCurrentTime();
    }
    return 0;
  }

  /** @param {number} secs */
  export async function seek(secs) {
    return player?.seek(secs);
  }

  /** @arg {VideoMarker[]} loadedMarkers */
  function markersLoaded(loadedMarkers) {
    markers = loadedMarkers;
    beats = loadedMarkers.map((marker) => marker.time);
  }
</script>

{#if !video}
  <VideoLoader {videoId} onLoaded={(v) => (video = v)} />
{:else if video.shortUUID}
  <div class="video">
    <PeertubeVideoPlayer
      bind:this={player}
      videoId={video.shortUUID}
      aspectRatio={video.aspectRatio || 1}
      {timeline}
      {markers}
      {beats}
    />
  </div>
{:else}
  Video missing shortUuid
{/if}

{#if markers === undefined && comboId && apiUser}
  <VideoMarkerLoader {comboId} {apiUser} onLoaded={markersLoaded}
  ></VideoMarkerLoader>
{/if}
