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
   */

  /** @type {Props} */
  let { track } = $props();

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
    // especially annoying on mobile if I don't do this
    if (document.hidden && isPlaying) {
      stopMusic();
      mutedDueToVisibility = true;
    } else if (!document.hidden && mutedDueToVisibility) {
      resumeMusic();
      mutedDueToVisibility = false;
    }
  }

  async function loadAndPlayTrack() {
    if (track) {
      resetMusic();
      await loadTrack(track.id);
      startMusic();
      $bpm = track.bpm;
    }
  }

  function startMusic() {
    if (musicNode) {
      resetMusic();
    }
    setChannelGain('music', musicContext.gain);
    musicNode = scheduleAudioOnChannel(track.id, 0, 'music');
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
