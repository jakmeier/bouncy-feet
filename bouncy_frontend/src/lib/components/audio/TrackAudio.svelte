<script>
  import { getContext, onDestroy, onMount } from 'svelte';
  import {
    setChannelGain,
    cleanupAudioNode,
    scheduleAudioOnChannel,
    loadTrack,
    initAudioContext,
  } from '$lib/stores/Audio';
  import { beatStart, bpm } from '$lib/stores/Beat';

  /**
   * @typedef {Object} Props
   * @property {Song} track
   * @property {()=>void} [onEnded]
   * @property {(durationMs: number)=>void} [onDuration]
   */

  /** @type {Props} */
  let { track, onEnded, onDuration } = $props();

  let musicContext = getContext('music');
  let isPlaying = $state(false);

  export function resetTrack() {
    resetMusic();
    startMusic();
    isPlaying = true;
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
    await loadAndPlayTrack();
    isPlaying = true;

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
      const audioBuffer = await loadTrack(selectedTrack.id);
      if (!audioBuffer || track !== selectedTrack) return;
      onDuration?.(audioBuffer.duration * 1000);
      startMusic(selectedTrack);
      $bpm = selectedTrack.bpm;
    }
  }

  /** @param {Song} selectedTrack */
  function startMusic(selectedTrack = track) {
    if (musicNode) {
      resetMusic();
    }
    setChannelGain('music', musicContext.gain);
    const source = scheduleAudioOnChannel(selectedTrack.id, 0, 'music');
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
    if (musicNode !== null) {
      cleanupAudioNode(musicNode, 'music');
      musicNode = null;
    }
  }

  $effect(() => {
    // keeping track on beat (poorly...)
    if ($beatStart) resetMusic();
  });
  $effect(() => {
    if (track) loadAndPlayTrack();
  });
</script>
