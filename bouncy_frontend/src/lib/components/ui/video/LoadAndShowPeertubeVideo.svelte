<script>
  import * as api from '$lib/peertube-openapi';
  import PeertubeVideoPlayer from './PeertubeVideoPlayer.svelte';
  import VideoBeatsLoader from './VideoBeatsLoader.svelte';
  import VideoLoader from './VideoLoader.svelte';
  import VideoMarkerLoader from './VideoMarkerLoader.svelte';

  /**
   * @typedef {Object} Props
   * @prop {number|string} videoId
   * @property {VideoTimelineConfig} [timeline]
   * @property {number} [comboId]
   * @property {ApiUser} [apiUser]
   * @property {api.VideoDetails | undefined} [video]
   * @property {Beat[]} [beats] -- bindable
   */

  /** @type {Props}*/
  let {
    videoId,
    timeline,
    comboId,
    apiUser,
    video = $bindable(),
    beats = $bindable(),
  } = $props();

  /** @type {PeertubeVideoPlayer | undefined}*/
  let player = $state();
  /** @type {VideoMarker[] | undefined} */
  let timestampMarkers = $state();

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

  export async function play() {
    return player?.play();
  }
  export async function pause() {
    return player?.pause();
  }

  /** @arg {VideoMarker[]} loadedMarkers */
  function markersLoaded(loadedMarkers) {
    timestampMarkers = loadedMarkers;
  }

  /** @arg {Beat[]} loadedBeats */
  function beatsLoaded(loadedBeats) {
    beats = loadedBeats;
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
      markers={timestampMarkers}
      {beats}
    />
  </div>
{:else}
  Video missing shortUuid
{/if}

{#if timestampMarkers === undefined && comboId && apiUser}
  <VideoMarkerLoader {comboId} {apiUser} onLoaded={markersLoaded}
  ></VideoMarkerLoader>
{/if}

{#if comboId}
  <VideoBeatsLoader {comboId} {apiUser} onLoaded={beatsLoaded}
  ></VideoBeatsLoader>
{/if}
