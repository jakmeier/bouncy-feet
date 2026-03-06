<script>
  import * as api from '$lib/peertube-openapi';
  import PeertubeVideoPlayer from './PeertubeVideoPlayer.svelte';
  import VideoLoader from './VideoLoader.svelte';
  import VideoMarkerLoader from './VideoMarkerLoader.svelte';

  /**
   * @typedef {Object} Props
   * @prop {number|string} videoId
   * @property {VideoTimelineConfig} [timeline]
   * @property {number} [comboId]
   * @property {ApiUser} [apiUser]
   * @property {api.VideoDetails | undefined} [video]
   * @property {VideoMarker[]} [extraMarkers] -- added to those loaded externally
   */

  /** @type {Props}*/
  let {
    videoId,
    timeline,
    comboId,
    apiUser,
    video = $bindable(),
    extraMarkers,
  } = $props();

  /** @type {PeertubeVideoPlayer | undefined}*/
  let player = $state();
  /** @type {VideoMarker[]} */
  let loadedMarkers = $state([]);

  /** @type {VideoMarker[]} */
  let rawMarkers = $derived([...(extraMarkers || []), ...loadedMarkers]);

  /** @type {VideoMarker[] | undefined} */
  const instantMarkers = $derived(
    // TODO: make custom timestamps useful, with icons etc
    rawMarkers?.filter((m) => !m.duration)
  );

  /** @type {number[] | undefined} */
  const beats = $derived(
    // for now, the only markers with duration are beats - later maybe also step ranges etc
    rawMarkers?.flatMap((marker) => {
      const markerBeats = [];
      if (marker.duration && marker.interval) {
        for (
          var t = 0;
          t < marker.duration && markerBeats.length < 10_000;
          t += marker.interval
        ) {
          markerBeats.push(marker.time + t);
        }
      }
      return markerBeats;
    })
  );

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

  /** @arg {VideoMarker[]} markers */
  function markersLoaded(markers) {
    loadedMarkers = markers;
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
      markers={instantMarkers}
      {beats}
    />
  </div>
{:else}
  Video missing shortUuid
{/if}

{#if rawMarkers === undefined && comboId && apiUser}
  <VideoMarkerLoader {comboId} {apiUser} onLoaded={markersLoaded}
  ></VideoMarkerLoader>
{/if}
