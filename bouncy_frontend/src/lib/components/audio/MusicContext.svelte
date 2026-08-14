<script>
  import TrackAudio from '$lib/components/audio/TrackAudio.svelte';
  import { setChannelGain } from '$lib/stores/Audio';
  import { songs } from '$lib/stores/Songs';
  import { setContext, tick } from 'svelte';
  import { readable } from 'svelte/store';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /** @type {Song | undefined} */
  let track = $state(undefined);
  let autoplayTrack = $state(true);
  /** @type {TrackAudio | undefined} */
  let audio = $state();

  let titleSetter = (_) => {};
  const songTitle = readable('-', (set) => {
    titleSetter = set;
  });

  let authorSetter = (_) => {};
  const songAuthor = readable('-', (set) => {
    authorSetter = set;
  });

  let gain = $state(1.0);
  /** @type {number | undefined} */
  let trackDurationMs = $state();
  const trackEndedListeners = new Set();

  /** @param {string} newTrackId */
  async function setTrack(newTrackId) {
    autoplayTrack = true;
    trackDurationMs = undefined;
    track = songs.get(newTrackId);
    titleSetter(track?.title || '-');
    authorSetter(track?.author || '-');
    await tick();
  }

  /**
   * Load a track without starting it. The track component is retained so old
   * preview callers can continue to use setTrack's autoplay behavior.
   * @param {string} songId
   * @returns {Promise<AudioBuffer | undefined>}
   */
  async function prepareTrack(songId) {
    autoplayTrack = false;
    trackDurationMs = undefined;
    track = songs.get(songId);
    titleSetter(track?.title || '-');
    authorSetter(track?.author || '-');
    await tick();
    return audio?.prepareTrack();
  }

  /** @param {number} timestamp @param {number} [playbackStartMs] */
  function startTrackAt(timestamp, playbackStartMs = 0) {
    audio?.startTrackAt(timestamp, playbackStartMs);
  }

  /** @param {number} timestamp @param {number} [playbackStartMs] */
  function resetTrackAt(timestamp, playbackStartMs = 0) {
    audio?.resetTrackAt(timestamp, playbackStartMs);
  }

  function resetTrack() {
    audio?.resetTrack();
  }

  function stopTrack() {
    audio?.stopMusic();
  }

  function resumeTrack() {
    audio?.resumeMusic();
  }

  /** @param {number} vol */
  function setVolume(vol) {
    gain = vol;
  }

  /** @param {()=>void} listener */
  function onTrackEnded(listener) {
    trackEndedListeners.add(listener);
    return () => trackEndedListeners.delete(listener);
  }

  /** @param {number} durationMs */
  function trackDurationLoaded(durationMs) {
    trackDurationMs = durationMs;
  }

  function trackEnded() {
    for (const listener of trackEndedListeners) listener();
  }

  setContext('music', {
    setTrack,
    prepareTrack,
    startTrackAt,
    resetTrackAt,
    resetTrack,
    stopTrack,
    resumeTrack,
    setVolume,
    onTrackEnded,
    songTitle,
    songAuthor,
    get gain() {
      return gain;
    },
    get trackOn() {
      return audio?.trackIsOn();
    },
    get trackStartTimestamp() {
      return audio?.trackStartTimestamp();
    },
    get trackDurationMs() {
      return trackDurationMs;
    },
  });

  $effect(() => {
    setChannelGain('music', gain);
  });
</script>

{#if track}
  <TrackAudio
    bind:this={audio}
    {track}
    autoplay={autoplayTrack}
    onEnded={trackEnded}
    onDuration={trackDurationLoaded}
  />
{/if}

{@render children?.()}
