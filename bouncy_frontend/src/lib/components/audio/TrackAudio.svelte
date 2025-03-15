<script>
  import { onDestroy, onMount } from 'svelte';
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
  export function resetTrack() {
    resetMusic();
    startMusic();
  }

  export function stopMusic() {
    setChannelGain('music', 0.0);
  }

  let initialized = $state(false);

  /**
   * batches of connected audio nodes that should be disconnected at some point
   * @type {AudioBufferSourceNode[]}
   */
  let connectedNodes = [];

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
    setChannelGain('music', 1.0);
    const node = scheduleAudioOnChannel(track.id, 0, 'music');
    connectedNodes.push(node);
  }

  function resetMusic() {
    for (const node of connectedNodes) {
      cleanupAudioNode(node, 'music');
    }
    connectedNodes = [];
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
