<script>
  import { asset } from '$app/paths';
  import { onMount } from 'svelte';
  import { getUserContext } from '$lib/stores/context';
  import Arrow from '../svg/Arrow.svelte';
  import UnstyledButton from '../UnstyledButton.svelte';
  import { beatToMarkers } from '$lib/video_utils';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import InstructorAvatar from '$lib/components/avatar/InstructorAvatar.svelte';
  import { LEFT_RIGHT_COLORING } from '$lib/constants';

  /** @typedef {{ time: number, label: string, icon: string }} Marker */
  /**
   * @typedef {Object} Props
   * @property {string} peertubeUrl
   * @property {number} aspectRatio
   * @property {Beat[]} [beats] - Array of beat timestamps in ms
   * @property {Marker[]} [markers] - Array of markers to show on the timeline
   * @property {PoseWrapper[]} [poses]
   * @property {boolean} [muted]
   * @property {VideoTimelineConfig} [timeline]
   */

  /** @type Props */
  let {
    peertubeUrl,
    beats = [],
    markers = [],
    muted = false,
    timeline = undefined,
    aspectRatio,
    poses = [],
  } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let isPlaying = $state(false);
  let duration = $state(0);
  let currentTime = $state(0);

  let iframeOverlay = $state();
  let iframe = $state();
  let player = $state();
  let magnifierWidth = $state();

  let showSpeedControl = $state(true);
  let playbackRates = $state([0.25, 0.5, 1.0]);

  const magnifiedTimeRangeSec = 2;
  const magnifiedPxPerSec = $derived(magnifierWidth / magnifiedTimeRangeSec);

  /** @type {{t: number, text: string}[]} */
  const beatMarkers = $derived(beatToMarkers(beats));

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
    currentTime = secs;
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
  function seekTo(time, snap = true) {
    const maxSnap = 500;
    const snapped = snap ? snapToBeat(time * 1000) : time * 1000;
    if (Math.abs(time * 1000 - snapped) <= maxSnap) {
      player.seek(snapped / 1000);
    } else {
      player.seek(time);
    }
  }

  /**
   * @param {number} targetMs
   * @returns {number}
   */
  function snapToBeat(targetMs) {
    if (beatMarkers.length === 0) {
      return targetMs;
    }
    return beatMarkers
      .map((m) => m.t)
      .reduce((prev, curr) =>
        Math.abs(curr - targetMs) < Math.abs(prev - targetMs) ? curr : prev
      );
  }

  let dragging = false;
  let dragStartX = 0;
  let dragStartTime = 0;

  /** @param {PointerEvent} e */
  function onPointerDown(e) {
    dragging = true;
    dragStartX = e.clientX;
    dragStartTime = currentTime;

    // @ts-ignore
    e.currentTarget?.setPointerCapture(e.pointerId);
  }

  /** @param {PointerEvent} e */
  function onPointerMove(e) {
    if (!dragging) return;
    seekToDragPosition(e.clientX, false);
  }

  /**
   * @param {number} clientX
   * @param {boolean} snap
   */
  function seekToDragPosition(clientX, snap) {
    const dx = clientX - dragStartX;
    const dt = dx / magnifiedPxPerSec;

    let newTime = dragStartTime - dt;
    newTime = Math.max(0, Math.min(duration, newTime));
    seekTo(newTime, snap);
  }

  /** @param {PointerEvent} e */
  function onPointerUp(e) {
    seekToDragPosition(e.clientX, true);
    dragging = false;

    try {
      // @ts-ignore
      e.currentTarget?.releasePointerCapture(e.pointerId);
    } catch {}
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

  function seekToPrevBeat() {
    const prev = beatMarkers.findLastIndex(
      (m) => m.t - currentTime * 1000 < -0.001
    );
    if (prev !== -1) {
      seekTo(beatMarkers[prev].t / 1000);
    }
  }

  function seekToNextBeat() {
    const prev = beatMarkers.findIndex((m) => m.t - currentTime * 1000 > 0.001);
    if (prev !== -1) {
      seekTo(beatMarkers[prev].t / 1000);
    }
  }

  /**
   * @param {number} indexChange
   */
  async function changePlayback(indexChange) {
    const current = await player.getPlaybackRate();
    const index = playbackRates.indexOf(current);
    const newIndex =
      (index + indexChange + playbackRates.length) % playbackRates.length;
    await player.setPlaybackRate(playbackRates[newIndex]);
  }
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
      allow="autoplay"
      bind:this={iframe}
    ></iframe>
  </div>

  {#if showSpeedControl}
    <div
      class="config-buttons"
      class:speed-control-inlined-timeline={timeline?.position === 'inline'}
    >
      <!-- <div class="mirror-button">
        <UnstyledButton onClick={() => {}}>
          <img src={asset('/img/symbols/bf_mirror.svg')} alt="mirror" />
        </UnstyledButton>
      </div> -->
      <div class="speed-button">
        <UnstyledButton onClick={() => changePlayback(-1)}>
          <img src={asset('/img/symbols/bf_slower.svg')} alt="slower" />
        </UnstyledButton>
      </div>
    </div>
  {/if}

  {#if !isPlaying}
    <div class="overlay-controls">
      <button class="play-button" onclick={togglePlay}>
        <img src={asset('/img/symbols/bf_play.svg')} alt="bf_eye" />
      </button>
    </div>
  {/if}
</div>

{#if timeline?.beatCounts && duration > 0}
  <div
    class="counts"
    class:with_skeletons={poses.length > 0}
    class:no_skeletons={!poses || poses.length === 0}
  >
    <UnstyledButton onClick={seekToPrevBeat}>
      <div class="arrow left">
        <Arrow color="var(--theme-neutral-white)" />
      </div>
    </UnstyledButton>

    <div
      class="counts-magnifier-bar"
      bind:clientWidth={magnifierWidth}
      onpointerdown={onPointerDown}
      onpointermove={onPointerMove}
      onpointerup={onPointerUp}
      onpointercancel={onPointerUp}
    >
      <div
        class="counts-magnifier-bar-content"
        style="width: {magnifiedPxPerSec *
          duration}px; transform: translate({-magnifiedPxPerSec * currentTime +
          magnifierWidth / 2}px);"
      >
        {#each beatMarkers as marker, i}
          <div
            class="magnified-beat-count"
            class:highlighted={Math.abs(marker.t - currentTime * 1000) < 0.01}
            style="transform: translate(calc({(marker.t / 1000) *
              magnifiedPxPerSec}px - 0.5rem - 2.5px));"
          >
            {marker.text}
          </div>

          {#if poses.length > i}
            <div
              class="skeleton"
              style="transform: translate(calc({(marker.t / 1000) *
                magnifiedPxPerSec}px - 0.5rem - 2.5px));"
            >
              <div class="avatar-container">
                {#if poses[i]}
                  <InstructorAvatar
                    avatarSize={1.0}
                    skeleton={poses[i].skeleton()}
                    instructorStyle={LEFT_RIGHT_COLORING}
                  />
                {/if}
              </div>
            </div>
          {/if}
        {/each}
      </div>
    </div>

    <UnstyledButton onClick={seekToNextBeat}>
      <div class="arrow right">
        <Arrow color="var(--theme-neutral-white)" />
      </div>
    </UnstyledButton>
  </div>
{/if}

{#if timeline && duration > 0}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class:external_timeline={timeline.position === 'external'}
    class:inlined_timeline={timeline.position === 'inline'}
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

    {#each beatMarkers as marker}
      <div
        class="beat-marker"
        style="left: {(marker.t / 1000 / duration) * 100}%"
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
    z-index: 11;
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

  .counts {
    position: relative;
    display: grid;
    grid-template-columns: 1.2rem auto 1.2rem;
    margin: 1rem 0;
    height: 2rem;
  }
  .with_skeletons.counts {
    height: 3rem;
  }

  .arrow {
    max-width: 1.2rem;
    max-height: 2rem;
  }
  .left {
    rotate: 90deg;
  }
  .right {
    rotate: -90deg;
  }

  .with_skeletons .arrow {
    max-height: 3rem;
  }

  .counts-magnifier-bar {
    position: relative;
    height: 100%;
    width: 75%;
    margin: auto;
    overflow: hidden;
    background-color: var(--theme-neutral-almost-black);
    border-radius: 1rem;

    /* Avoid touch scrolling the page */
    touch-action: none;
    user-select: none;
    cursor: grab;
  }

  .counts-magnifier-bar:active {
    cursor: grabbing;
  }

  .counts-magnifier-bar-content {
    position: relative;
    left: 0;
    height: 100%;
    display: flex;
  }

  .magnified-beat-count {
    position: absolute;
    left: 0;
    align-self: center;
    font-size: var(--font-small);
    color: var(--theme-main);
    width: 1rem;
    height: 1rem;
    text-align: center;
    align-content: center;
  }

  .with_skeletons .magnified-beat-count {
    top: 2rem;
    color: var(--theme-neutral-darker-gray);
  }

  .skeleton {
    position: absolute;
    left: -0.5rem;
    top: 0;
    height: 2rem;
  }

  .avatar-container {
    position: relative;
    /* margin: -50% 0; */
    width: auto;
    height: 100%;
  }

  .no_skeletons .highlighted {
    background-color: var(--theme-neutral-darker-gray);
    border-radius: 50%;
    padding: 5px;
    width: 1rem;
    height: 1rem;
    text-align: center;
  }
  .with_skeletons .highlighted {
    color: var(--theme-main);
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

  .config-buttons {
    position: absolute;
    bottom: 1rem;
    height: 2rem;
    overflow: hidden;
    z-index: 12;
    display: grid;
    width: calc(100% - 1rem);
    margin: 0 0.5rem;
    grid-template-columns: min-content min-content;
    justify-content: right;
    gap: 1rem;
  }

  .speed-control-inlined-timeline {
    bottom: 3rem;
  }

  /* .mirror-button, */
  .speed-button {
    height: 2rem;
    width: 2rem;
  }

  img {
    height: 100%;
  }
</style>
