<script>
  import { run } from 'svelte/legacy';

  import { onDestroy, onMount } from 'svelte';
  import BackgroundTask from '../BackgroundTask.svelte';
  import {
    setChannelGain,
    cleanupAudioNode,
    loadBeatSounds,
    scheduleAudioOnChannel,
    initAudioContext,
  } from '$lib/stores/Audio';
  import { beatStart, timeBetweenMoves } from '$lib/stores/Beat';

  /**
   * @typedef {Object} Props
   * @property {boolean} [isOn]
   */

  /** @type {Props} */
  let { isOn = false } = $props();

  let initialized = $state(false);

  let kickAudioFiles = ['kick', 'kick2'];
  let halfBeat = 0;
  let isPlaying = false;
  /**
   * batches of connected audio nodes that should be disconnected at some point
   * @type {AudioBufferSourceNode[][]}
   */
  let connectedNodes = [];

  onMount(async () => {
    await initAudioContext();
    await loadBeatSounds();
    if (isOn) startAudio();
    initialized = true;
  });

  onDestroy(() => {
    stopAudio();
    resetAudio();
  });

  /**
   * @param {number} time
   * @param {string} id
   * @return {AudioBufferSourceNode}
   */
  function scheduleNote(time, id) {
    return scheduleAudioOnChannel(id, time, 'audio-component');
  }

  /**
   * @param {number} start
   * @param {number} beatDuration
   * @param {number} numBeats
   * @return {AudioBufferSourceNode[]}
   */
  function scheduleNBeats(start, beatDuration, numBeats) {
    let time = start;
    let nodes = [];
    let now = performance.now();
    for (let i = 0; i < numBeats; i++) {
      if (time < now) {
        time += beatDuration;
        halfBeat++;
        continue;
      }

      const fileName = kickAudioFiles[halfBeat % 2];
      const node = scheduleNote(time, fileName);
      nodes.push(node);
      time += beatDuration;
      halfBeat++;
    }
    return nodes;
  }

  function startAudio() {
    if (isPlaying) return;
    setChannelGain('audio-component', 1.0);
    isPlaying = true;
  }

  function stopAudio() {
    if (!isPlaying) return;
    setChannelGain('audio-component', 0.0);
    isPlaying = false;
  }

  function resetAudio() {
    for (const nodes of connectedNodes) {
      for (const node of nodes) {
        cleanupAudioNode(node, 'audio-component');
      }
    }
    connectedNodes = [];
  }

  function onFrame() {
    if (!isPlaying) return;
    const batchSize = 8;
    const msPerBatch = batchSize * $timeBetweenMoves;

    while (nextNoteTime < performance.now() + msPerBatch) {
      const nodes = scheduleNBeats(nextNoteTime, $timeBetweenMoves, batchSize);
      connectedNodes.push(nodes);
      nextNoteTime += msPerBatch;
    }

    while (connectedNodes.length > 3) {
      const nodes = connectedNodes.shift();
      for (const node of nodes) {
        cleanupAudioNode(node, 'audio-component');
      }
    }
  }
  run(() => {
    initialized && (isOn ? startAudio() : stopAudio());
  });
  run(() => {
    initialized && $timeBetweenMoves && resetAudio();
  });
  /** @type {number} ms performance timestamp */
  run(() => {
    $beatStart, resetAudio();
  });
  let nextNoteTime;
  run(() => {
    nextNoteTime = $beatStart;
  });
</script>

<BackgroundTask {onFrame}></BackgroundTask>
