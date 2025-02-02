<script>
  import { onDestroy, onMount } from 'svelte';
  import {
    setChannelGain,
    cleanupAudioNode,
    scheduleAudioOnChannel,
    loadTrack,
  } from '$lib/stores/Audio';
  import { beatStart, timeBetweenMoves } from '$lib/stores/Beat';

  export let isOn = false;
  /** @type {string} */
  export let track;

  let initialized = false;
  $: initialized && (isOn ? startAudio() : stopAudio());
  // $: initialized && $timeBetweenMoves && resetAudio();

  /** @type {number} ms performance timestamp */
  $: $beatStart, resetAudio();

  let isPlaying = false;
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
    stopAudio();
    resetAudio();
  });

  $: track, loadAndPlayTrack();
  async function loadAndPlayTrack() {
    if (track !== '') {
      await loadTrack(track);
      startAudio();
    }
  }

  /**
   * @param {number} time
   * @param {string} id
   * @return {AudioBufferSourceNode}
   */
  function scheduleSong(time) {
    return scheduleAudioOnChannel(track, time, 'audio-component');
  }

  function startAudio() {
    if (isPlaying) return;
    setChannelGain('audio-component', 1.0);
    isPlaying = true;
    const node = scheduleSong(0);
    connectedNodes.push(node);
  }

  function stopAudio() {
    if (!isPlaying) return;
    setChannelGain('audio-component', 0.0);
    isPlaying = false;
  }

  function resetAudio() {
    for (const node of connectedNodes) {
      cleanupAudioNode(node, 'audio-component');
    }
    connectedNodes = [];
  }
</script>
