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
   * @property {boolean} [isOn]
   * @property {Song} track
   */

  /** @type {Props} */
  let { isOn = false, track } = $props();

  let musicContext = getContext('music');

  export function resetTrack() {
    resetMusic();
    startMusic();
  }

  export function stopMusic() {
    setChannelGain('music', 0.0);
  }

  export function resumeMusic() {
    setChannelGain('music', musicContext.gain);
  }

  let initialized = $state(false);

  /**
   * Currently playing music
   * @type {AudioBufferSourceNode | null}
   */
  let musicNode = null;

  onMount(async () => {
    await initAudioContext();
    await loadAndPlayTrack();
    initialized = true;
  });

  onDestroy(() => {
    stopMusic();
    resetMusic();
  });

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
    initialized && (isOn ? startMusic() : stopMusic());
  });

  $effect(() => {
    // keeping track on beat (poorly...)
    if ($beatStart) resetMusic();
  });
  $effect(() => {
    if (track) loadAndPlayTrack();
  });
</script>
