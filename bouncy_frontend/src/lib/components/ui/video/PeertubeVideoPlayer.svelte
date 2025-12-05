<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { onMount } from 'svelte';
  import Symbol from '../Symbol.svelte';
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
  <div class="wrapper">
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
  />
{/if}

<style>
  .wrapper {
    width: 300px;
    aspect-ratio: 9 / 16;
    display: grid;
    place-items: center;
  }
</style>
