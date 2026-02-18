<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { onMount } from 'svelte';
  import Symbol from '../Symbol.svelte';
  import PeertubePlayer from './PeertubePlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} videoId
   * @property {number} aspectRatio
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {VideoMarker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   * @property {"inline"|"external"} [timeline]
   * @property {boolean} [isPrivate]
   * @property {number} [delayLoadingMs]
   */

  /** @type Props */
  let {
    videoId,
    beats = [],
    markers = [],
    muted = false,
    timeline,
    isPrivate = false,
    delayLoadingMs = 0,
    aspectRatio,
  } = $props();

  /** @type {PeertubePlayer | undefined} */
  let player = $state();
  let deferred = $state(delayLoadingMs > 0);

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

  export function forceLoad() {
    deferred = false;
  }

  const peertubeUrl = $derived(
    PUBLIC_BF_PEERTUBE_URL +
      '/videos/embed/' +
      videoId +
      '?api=1&warningTitle=0&controlBar=0&peertubeLink=0&controls=0&requiresAuth=' +
      (isPrivate ? '1' : '0')
  );

  onMount(() => {
    if (deferred) {
      setTimeout(() => (deferred = false), delayLoadingMs);
    }
  });
</script>

{#if deferred}
  <div class="placeholder" style="aspect-ratio: {aspectRatio};">
    <Symbol size={100} styleClass="rotating">refresh</Symbol>
  </div>
{:else}
  <PeertubePlayer
    bind:this={player}
    {peertubeUrl}
    {beats}
    {markers}
    {muted}
    {timeline}
    {aspectRatio}
  />
{/if}

<style>
  .placeholder {
    width: 300px;
    display: grid;
    place-items: center;
  }
</style>
