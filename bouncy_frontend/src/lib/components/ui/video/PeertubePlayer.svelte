<script>
  import { asset, base } from '$app/paths';
  import { onMount } from 'svelte';
  import { getUserContext } from '$lib/stores/context';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} peertubeUrl
   * @property {number} aspectRatio
   * @property {number[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {boolean} [muted]
   * @property {"inline"|"external"} [timeline]
   */

  /** @type Props */
  let {
    peertubeUrl,
    beats = [],
    markers = [],
    muted = false,
    timeline = undefined,
    aspectRatio,
  } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let isPlaying = $state(false);
  let duration = $state(0);
  let currentTime = $state(0);

  let iframeOverlay = $state();
  let iframe = $state();
  let player = $state();

  export function play() {
    if (player) {
      player.play();
      isPlaying = true;
      iframeOverlay.focus();
    }
  }

  export function pause() {
    if (player) {
      player.pause();
      isPlaying = false;
    }
  }

  /** @returns {Promise<number>} seconds */
  export async function getCurrentTime() {
    return player.getCurrentTime();
  }

  /** @param {number} secs */
  export async function seek(secs) {
    return player.seek(secs);
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

  async function togglePlay() {
    if (!(await player.isPlaying())) {
      player.play();
      isPlaying = true;
    } else {
      player.pause();
      isPlaying = false;
    }
  }

  /**
   * @param {KeyboardEvent} event
   */
  async function iframeOverlayKeyDown(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      // event.preventDefault();
      await togglePlay();
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

  onMount(async () => {
    // Importing this normally fails with `window not defined`
    const { PeerTubePlayer } = await import('@peertube/embed-api');

    player = new PeerTubePlayer(iframe);
    // set up refreshed token forwarding
    player.addEventListener('authFailed', async () => {
      if (userCtx.fullUser) {
        await player.setAuthToken(
          userCtx.fullUser?.pwaAuth.peerTubeToken?.access_token
        );
      } else {
        // TODO: should show a login mask in place of the video
        console.log(
          'authFailed event listener triggered but user is not logged in'
        );
      }
    });
    // set current token once to resolve
    // await player.setAuthToken(userCtx.pwaAuth.peerTubeToken?.access_token);

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

<div class="video-wrapper" style="--video-ratio: {aspectRatio}">
  <div class="iframe-wrapper">
    <div
      class="iframe-overlay"
      onclick={togglePlay}
      onkeydown={iframeOverlayKeyDown}
      bind:this={iframeOverlay}
      role="switch"
      aria-checked={isPlaying}
      tabindex="0"
    ></div>
    <iframe
      title="video"
      src={peertubeUrl}
      frameborder="0"
      sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
      bind:this={iframe}
    ></iframe>
  </div>
  {#if !isPlaying}
    <div class="overlay-controls">
      <button class="play-button" onclick={togglePlay}>
        <img src="{base}/img/symbols/bf_play.svg" alt="bf_eye" />
      </button>
    </div>
  {/if}
</div>

{#if timeline}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class:external_timeline={timeline === 'external'}
    class:inlined_timeline={timeline === 'inline'}
    onclick={(e) => {
      const rect = e.currentTarget.getBoundingClientRect();
      const percent = (e.clientX - rect.left) / rect.width;
      seekTo(percent * duration);
    }}
  >
    <div
      class="progress"
      style="width: {(currentTime / duration) * 100}%"
    ></div>

    {#each beats as t}
      <div
        class="beat-marker"
        style="left: {(t / 1000 / duration) * 100}%"
      ></div>
    {/each}

    {#each markers as marker}
      <div
        class="custom-marker"
        title={marker.label}
        style="left: {(marker.time / 1000 / duration) * 100}%"
      >
        {#if marker.icon.length > 0}
          <img
            class="icon"
            src={asset(`/icons/${marker.icon}.svg`)}
            alt="Bouncy Feet Logo"
          />
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .video-wrapper {
    position: relative;
    max-width: 100vw;
    max-height: 100vh;
    width: 100%;
    height: 100%;
    aspect-ratio: var(--video-ratio);
    display: flex;
    align-items: center;
    justify-content: center;
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

  .external_timeline {
    position: relative;
    height: 2rem;
    background: var(--theme-neutral-light);
    margin-top: 1.125rem;
    cursor: pointer;
    border-radius: 1rem;
    overflow: hidden;
  }

  .inlined_timeline {
    position: absolute;
    bottom: 1rem;
    height: 2rem;
    background: var(--theme-neutral-light);
    cursor: pointer;
    border-radius: 1rem;
    overflow: hidden;
    width: calc(100% - 1rem);
    z-index: 11;
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
    /* Use the known aspect ratio to scale */
    width: 100%;
    height: 100%;
    /* scale to cover container */
    display: flex;
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
    min-width: auto;
    min-height: auto;
    width: 100%;
    height: 100%;
    border: 0;
    object-fit: contain;
  }
</style>
