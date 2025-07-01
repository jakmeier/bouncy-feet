<script>
  import { base } from '$app/paths';
  // A custom video player for Bouncy Feet, that's aware of music beats and just
  // generally styled in theme.
  //
  // Usage: Put <source> as children
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import CornerMarker from '../CornerMarker.svelte';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} path
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   */

  /** @type Props */
  let { path, beats = [], markers = [], muted = false } = $props();

  let videoElement = $state();
  let isPlaying = $state(false);
  let duration = $state(0);
  let currentTime = $state(0);
  let videoExists = $state(false);
  let videoLoading = $state(true);

  export function play() {
    if (videoElement) {
      videoElement.play();
    }
  }

  const seekTime = writable(0);

  function togglePlay() {
    if (videoElement.paused) {
      videoElement.play();
    } else {
      videoElement.pause();
    }
  }

  function seekTo(time) {
    const snapped = snapToBeat(time * 1000);
    videoElement.currentTime = snapped / 1000;
  }

  function snapToBeat(targetMs) {
    if (beats.length === 0) {
      return targetMs;
    }
    return beats.reduce((prev, curr) =>
      Math.abs(curr - targetMs) < Math.abs(prev - targetMs) ? curr : prev
    );
  }

  onMount(() => {
    videoElement.addEventListener('loadedmetadata', () => {
      duration = videoElement.duration;
    });
    videoElement.addEventListener('timeupdate', () => {
      currentTime = videoElement.currentTime;
    });
    videoElement.addEventListener('play', () => (isPlaying = true));
    videoElement.addEventListener('pause', () => (isPlaying = false));
  });
</script>

<div class="video-wrapper">
  <CornerMarker>
    <video
      bind:this={videoElement}
      controls={false}
      preload="auto"
      playsinline
      webkit-playsinline
      defaultmuted={muted}
      {muted}
      onloadedmetadata={() => {
        videoExists = true;
        videoLoading = false;
      }}
      onclick={togglePlay}
    >
      <source
        src={path}
        type="video/mp4"
        onerror={() => {
          videoLoading = false;
          videoExists = false;
        }}
        onsuspend={() => {
          videoLoading = false;
          videoExists = false;
        }}
      />
    </video>
  </CornerMarker>
  {#if !isPlaying}
    <div class="overlay-controls">
      <button class="play-button" onclick={togglePlay}>
        <img src="{base}/img/symbols/bf_play.svg" alt="bf_eye" />
      </button>
    </div>
  {/if}
</div>

<div
  class="timeline"
  onclick={(e) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const percent = (e.clientX - rect.left) / rect.width;
    seekTo(percent * duration);
  }}
>
  <div class="progress" style="width: {(currentTime / duration) * 100}%"></div>

  {#each beats as b}
    <div class="beat-marker" style="left: {(b / 1000 / duration) * 100}%"></div>
  {/each}

  {#each markers as m}
    <div
      class="custom-marker"
      title={m.label}
      style="left: {(m.time / duration) * 100}%"
      onclick={(e) => {
        e.stopPropagation();
        seekTo(m.time);
      }}
    >
      {m.icon}
    </div>
  {/each}
</div>

<style>
  .video-wrapper {
    position: relative;
    width: 100%;
    max-width: 800px;
    /* left: -0.5rem; */
  }

  video {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
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

  .beat-marker,
  .custom-marker {
    position: absolute;
    top: 0;
    height: 100%;
    width: 2px;
    background: var(--theme-neutral-black);
  }
</style>
