<script>
  import { getContext, onDestroy, onMount } from 'svelte';
  import {
    setChannelGain,
    cleanupAudioNode,
    scheduleAudioTrackOnChannel,
    loadTrack,
    initAudioContext,
  } from '$lib/stores/Audio';
  import { bpm } from '$lib/stores/Beat';

  /**
   * @typedef {Object} Props
   * @property {Song} track
   * @property {boolean} [autoplay]
   * @property {()=>void} [onEnded]
   * @property {(durationMs: number)=>void} [onDuration]
   */

  /** @type {Props} */
  let { track, autoplay = true, onEnded, onDuration } = $props();

  let musicContext = getContext('music');
  let isPlaying = $state(false);
  /** @type {number | undefined} */
  let scheduledStartTimestamp = $state();

  export function resetTrack() {
    resetTrackAt(performance.now(), 0);
  }

  /**
   * Load the selected track without making it audible.
   * @returns {Promise<AudioBuffer | undefined>}
   */
  export async function prepareTrack() {
    // prepareTrack may be called by a parent immediately after this component
    // is created, before this component's onMount hook has run.
    await initAudioContext();
    const selectedTrack = track;
    if (!selectedTrack) return undefined;
    const audioBuffer = await loadTrack(selectedTrack.id);
    if (!audioBuffer || track !== selectedTrack) return undefined;
    onDuration?.(audioBuffer.duration * 1000);
    return audioBuffer;
  }

  /**
   * Start the prepared track at a performance.now()-compatible timestamp.
   * @param {number} timestamp
   * @param {number} [playbackStartMs]
   */
  export function startTrackAt(timestamp, playbackStartMs = 0) {
    if (!track) return;
    startMusic(track, timestamp, playbackStartMs);
    isPlaying = true;
  }

  /** @returns {number | undefined} */
  export function trackStartTimestamp() {
    return scheduledStartTimestamp;
  }

  /**
   * Replace the current source and start at the requested song position.
   * @param {number} timestamp
   * @param {number} [playbackStartMs]
   */
  export function resetTrackAt(timestamp, playbackStartMs = 0) {
    resetMusic();
    startTrackAt(timestamp, playbackStartMs);
  }

  export function stopMusic() {
    setChannelGain('music', 0.0);
    isPlaying = false;
  }

  export function resumeMusic() {
    setChannelGain('music', musicContext.gain);
    isPlaying = true;
  }

  export function trackIsOn() {
    return isPlaying;
  }

  /**
   * Currently playing music
   * @type {AudioBufferSourceNode | null}
   */
  let musicNode = null;

  onMount(async () => {
    await initAudioContext();
    if (autoplay) {
      await loadAndPlayTrack();
      isPlaying = true;
    } else {
      await prepareTrack();
    }

    document.addEventListener('visibilitychange', visibilityHandler, false);
  });

  onDestroy(() => {
    stopMusic();
    resetMusic();
    document.removeEventListener('visibilitychange', visibilityHandler, false);
  });

  let mutedDueToVisibility = false;
  function visibilityHandler() {
    // Mute in the background without marking the source as stopped: a natural
    // end must still finish a challenge attempt.
    if (document.hidden && isPlaying) {
      setChannelGain('music', 0.0);
      mutedDueToVisibility = true;
    } else if (!document.hidden && mutedDueToVisibility) {
      resumeMusic();
      mutedDueToVisibility = false;
    }
  }

  async function loadAndPlayTrack() {
    const selectedTrack = track;
    if (selectedTrack) {
      resetMusic();
      const audioBuffer = await prepareTrack();
      if (!audioBuffer || track !== selectedTrack) return;
      startMusic(selectedTrack);
      $bpm = selectedTrack.bpm;
    }
  }

  /**
   * @param {Song} selectedTrack
   * @param {number} [timestamp]
   * @param {number} [playbackStartMs]
   */
  function startMusic(
    selectedTrack = track,
    timestamp = performance.now(),
    playbackStartMs = 0
  ) {
    if (musicNode) {
      resetMusic();
    }
    scheduledStartTimestamp = timestamp;
    setChannelGain('music', musicContext.gain);
    const source = scheduleAudioTrackOnChannel(
      selectedTrack.id,
      timestamp,
      'music',
      playbackStartMs / 1000
    );
    musicNode = source;
    source.onended = () => {
      // Resetting/replacing a song disconnects an old source. Only the active,
      // audible source ending naturally ends the challenge attempt.
      if (musicNode !== source) return;
      musicNode = null;
      const endedNaturally = isPlaying;
      isPlaying = false;
      if (endedNaturally) onEnded?.();
    };
  }

  function resetMusic() {
    scheduledStartTimestamp = undefined;
    if (musicNode !== null) {
      cleanupAudioNode(musicNode, 'music');
      musicNode = null;
    }
  }

  $effect(() => {
    if (track && autoplay) loadAndPlayTrack();
  });
</script>
