<script>
  import PeertubePlayer from './PeertubePlayer.svelte';
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} playlistId
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   * @property {boolean} [timeline]
   */

  /** @type Props */
  let {
    playlistId,
    beats = [],
    markers = [],
    muted = false,
    timeline,
  } = $props();

  let player = $state();
  let playlistPosition = $state(1);

  export function play() {
    if (player) {
      player.play();
    }
  }
</script>

<PeertubePlayer
  bind:this={player}
  peertubeUrl="{PUBLIC_BF_PEERTUBE_URL}/video-playlists/embed/{playlistId}?api=1&warningTitle=0&controlBar=0&peertubeLink=0&controls=0&playlistPosition={playlistPosition}&autoplay=0"
  {beats}
  {markers}
  {muted}
  {timeline}
/>
