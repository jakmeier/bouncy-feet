<script>
  import { onDestroy, onMount } from 'svelte';
  import {
    setChannelGain,
    cleanupAudioNode,
    scheduleAudioOnChannel,
    loadTrack,
  } from '$lib/stores/Audio';
  import { beatStart, bpm } from '$lib/stores/Beat';

  export let isOn = false;
  /** @type {Song} */
  export let track;
  export function resetTrack() {
    resetMusic();
    startMusic();
  }

  export function stopMusic() {
    setChannelGain('music', 0.0);
  }

  let initialized = false;
  $: initialized && (isOn ? startMusic() : stopMusic());
  // $: initialized && $timeBetweenMoves && resetAudio();

  /** @type {number} ms performance timestamp */
  $: $beatStart, resetMusic();

  /**
   * batches of connected audio nodes that should be disconnected at some point
   * @type {AudioBufferSourceNode[]}
   */
  let connectedNodes = [];

  onMount(async () => {
    await loadAndPlayTrack();
    initialized = true;
  });

  onDestroy(() => {
    stopMusic();
    resetMusic();
  });

  $: track, loadAndPlayTrack();
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
</script>
