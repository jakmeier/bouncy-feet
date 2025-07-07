<script>
  import { base } from '$app/paths';
  import { onMount } from 'svelte';
  import CornerMarker from '../CornerMarker.svelte';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} videoId
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   */

  /** @type Props */
  let { videoId, beats = [], markers = [], muted = false } = $props();

  let isPlaying = $state(false);
  let duration = $state(0);
  let currentTime = $state(0);

  let iframe = $state();
  let iframeWrapperWidth = $state(90);
  let iframeWrapperHeight = $state(160);
  let player = $state();

  export function play() {
    if (player) {
      player.play();
      isPlaying = true;
    }
  }

  async function togglePlay() {
    if (!(await player.isPlaying())) {
      // if (!isPlaying) {
      player.play();
      isPlaying = true;
    } else {
      player.pause();
      isPlaying = false;
    }
  }

  /**
   * @param {number} time in seconds
   */
  function seekTo(time) {
    const maxSnap = 1000;
    const snapped = snapToBeat(time * 1000);
    if (Math.abs(time * 1000 - snapped) <= maxSnap) {
      player.seek(snapped / 1000);
    } else {
      player.seek(time);
    }
  }

  /**
   * @param {number} targetMs
   */
  function snapToBeat(targetMs) {
    if (beats.length === 0) {
      return targetMs;
    }
    return beats.reduce((prev, curr) =>
      Math.abs(curr - targetMs) < Math.abs(prev - targetMs) ? curr : prev
    );
  }

  /**
   * @typedef {Object} PeerTubePlayerState
   * @property {number} position - Current playback position in seconds.
   * @property {number} volume - Volume level (0.0 to 1.0).
   * @property {string} duration - Total duration of the video (as stringified float).
   * @property {"playing" | "paused" | "ended"} playbackState - Current playback state.
   */

  onMount(async () => {
    // Importing this normally fails with `window not defined`
    const { PeerTubePlayer } = await import('@peertube/embed-api');

    player = new PeerTubePlayer(iframe);

    player.addEventListener(
      'playbackStatusChange',
      async () => (isPlaying = await player.isPlaying())
    );
    player.addEventListener(
      'playbackStatusUpdate',
      /**
       * @param {PeerTubePlayerState} data
       */
      (data) => {
        currentTime = data.position;
        duration = Number(data.duration);
      }
    );
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="video-wrapper">
  <CornerMarker>
    <div
      class="iframe-wrapper"
      bind:clientWidth={iframeWrapperWidth}
      bind:clientHeight={iframeWrapperHeight}
    >
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="iframe-overlay" onclick={togglePlay}></div>
      <!-- TODO: video title -->
      <iframe
        title="video"
        width="{iframeWrapperWidth}px"
        height="{iframeWrapperHeight}px"
        src="https://tube.bouncy-feet.ch/videos/embed/{videoId}?api=1&warningTitle=0&controlBar=0&peertubeLink=0&controls=0"
        frameborder="0"
        sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
        bind:this={iframe}
      ></iframe>
    </div>
  </CornerMarker>
  {#if !isPlaying}
    <div class="overlay-controls">
      <button class="play-button" onclick={togglePlay}>
        <img src="{base}/img/symbols/bf_play.svg" alt="bf_eye" />
      </button>
    </div>
  {/if}
</div>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="timeline"
  onclick={(e) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    seekTo(percent * duration);
  }}
>
  <div class="progress" style="width: {(currentTime / duration) * 100}%"></div>

  {#each beats as t}
    <div class="beat-marker" style="left: {(t / 1000 / duration) * 100}%"></div>
  {/each}

  {#each markers as marker}
    <div
      class="custom-marker"
      title={marker.label}
      style="left: {(marker.time / 1000 / duration) * 100}%"
    >
      <img
        class="icon"
        src="{base}/icons/{marker.icon}.svg"
        alt="Bouncy Feet Logo"
      />
    </div>
  {/each}
</div>

<style>
  .video-wrapper {
    position: relative;
    width: 100%;
    max-width: 800px;
    /* left: -0.5rem; */

    aspect-ratio: 9 / 16;
  }

  .overlay-controls {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;
  }

  .play-button {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: transparent;
    pointer-events: auto;
    cursor: pointer;
  }

  .timeline {
    position: relative;
    left: 0.25rem;
    height: 2rem;
    background: var(--theme-neutral-light);
    margin-top: 1.125rem;
    cursor: pointer;
    border-radius: 1rem;
    overflow: hidden;
  }

  .progress {
    height: 100%;
    background: var(--theme-main);
    width: 0%;
  }

  .custom-marker,
  .beat-marker {
    position: absolute;
    top: 0;
  }

  .beat-marker {
    height: 30%;
    width: 2px;
    background: #1d1d1b20;
  }

  .beat-marker:nth-child(odd) {
    height: 50%;
  }

  .icon {
    position: relative;
    left: -0.75rem;
    top: 0.25rem;
    width: 1.5rem;
  }

  .iframe-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
  }

  .iframe-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    /* transparent but clickable */
    background: rgba(0, 0, 0, 0);
    z-index: 10;
    cursor: pointer;
  }

  iframe {
    display: block;

    width: 100%;
    height: 100%;
    border: 0;
  }
</style>
