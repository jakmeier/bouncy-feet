<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import PeertubePlayer from './PeertubePlayer.svelte';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} videoId
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   * @property {boolean} [timeline]
   * @property {boolean} [isPrivate]
   */

  /** @type Props */
  let {
    videoId,
    beats = [],
    markers = [],
    muted = false,
    timeline,
    isPrivate = false,
  } = $props();

  let player = $state();

  export function play() {
    if (player) {
      player.play();
    }
  }

  export function pause() {
    if (player) {
      player.pause();
    }
  }

  export function addEventListener(event, listener) {
    if (player) {
      player.addEventListener(event, listener);
    }
  }

  export function removeEventListener(event, listener) {
    if (player) {
      player.removeEventListener(event, listener);
    }
  }

  const peertubeUrl = $derived(
    PUBLIC_BF_PEERTUBE_URL +
      '/videos/embed/' +
      videoId +
      '?api=1&warningTitle=0&controlBar=0&peertubeLink=0&controls=0&requiresAuth=' +
      (isPrivate ? '1' : '0')
  );
</script>

<PeertubePlayer
  bind:this={player}
  {peertubeUrl}
  {beats}
  {markers}
  {muted}
  {timeline}
/>
