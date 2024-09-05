<script>
  import { base } from '$app/paths';
  import { onDestroy, onMount } from 'svelte';
  import BackgroundTask from './BackgroundTask.svelte';
  import { audioContext, loadAudio, getAudio } from '$lib/stores/Audio';

  export let secondsPerNote = 0.5;
  export let isOn = false;
  export let voice = false;

  let initialized = false;
  $: initialized && (isOn ? startAudio() : stopAudio());
  $: initialized && secondsPerNote && resetAudio();

  let countAudioFiles = ['one', 'two', 'three', 'four'];
  let andAudioFiles = ['and_0', 'and_1', 'and_2'];
  let kickAudioFiles = ['kick', 'kick2'];
  /** @type {GainNode} */
  let audioOutput;
  /** @type {number} seconds in audio context, when the first unscheduled note
   * should be scheduled */
  let nextNoteTime = 0.0;
  let halfBeat = 0;
  let isPlaying = false;
  let slowNoteString = '    1 2 12341234';
  let fastNoteString = '    1 a 2 a 1a2a3a4a1a2a3a4a';
  $: noteString = secondsPerNote < 0.55 ? fastNoteString : slowNoteString;
  /**
   * batches of connected audio nodes that should be disconnected at some point
   * @type {AudioBufferSourceNode[][]}
   */
  let connectedNodes = [];

  onMount(async () => {
    audioOutput = new GainNode(audioContext);
    await Promise.all(countAudioFiles.map(loadAudioSource));
    await Promise.all(andAudioFiles.map(loadAudioSource));
    await Promise.all(kickAudioFiles.map(loadAudioSource));

    if (isOn) startAudio();
    nextNoteTime = audioContext.currentTime;
    initialized = true;
  });

  onDestroy(() => {
    stopAudio();
  });

  /**
   * @param {any} filename
   */
  async function loadAudioSource(filename) {
    const url = `${base}/audio/${filename}.mp3`;
    return loadAudio(filename, url);
  }

  /**
   * @param {number} time
   * @param {string} id
   * @return {AudioBufferSourceNode}
   */
  function scheduleNote(time, id) {
    const bufferSource = getAudio(id);
    if (bufferSource) {
      bufferSource.connect(audioOutput);
      bufferSource.start(time - audioContext.outputLatency);
    }
    if (audioContext.state === 'suspended') {
      // on a page reload, the audio context is usually prevented from starting
      // automatically, we have to wait for a user interaction.
      audioContext.resume();
    }
    return bufferSource;
  }

  /**
   * @param {number} start
   * @param {number} beatDuration
   * @param {string} notes
   * @return {AudioBufferSourceNode[]}
   */
  function scheduleNoteString(start, beatDuration, notes) {
    let time = start;
    let nodes = [];
    for (const note of notes) {
      if (time < audioContext.currentTime) {
        time += beatDuration;
        halfBeat++;
        continue;
      }
      // counts
      if (voice) {
        const count = noteAudioId(note);
        if (count) {
          const node = scheduleNote(time, count);
          nodes.push(node);
        }
      }
      // also add a kick
      const fileName = kickAudioFiles[halfBeat % 2];
      const node = scheduleNote(time, fileName);
      nodes.push(node);
      time += beatDuration;
      halfBeat++;
    }
    return nodes;
  }

  let andCounter = 0;
  /**
   * @param {string} note
   * @returns {string|undefined}
   */
  function noteAudioId(note) {
    if (note >= '0' && note <= '9') {
      return countAudioFiles[parseInt(note) - 1];
    }
    if (note === 'a') {
      andCounter++;
      return andAudioFiles[(andCounter - 1) % andAudioFiles.length];
    }
  }

  function startAudio() {
    if (isPlaying) return;
    audioOutput.connect(audioContext.destination);
    isPlaying = true;
  }

  function stopAudio() {
    if (!isPlaying) return;
    audioOutput.disconnect(audioContext.destination);
    isPlaying = false;
  }

  function resetAudio() {
    for (const nodes of connectedNodes) {
      for (const node of nodes) {
        node.disconnect(audioOutput);
      }
    }
    connectedNodes = [];
    nextNoteTime = audioContext.currentTime;
  }

  function onFrame() {
    if (!isPlaying) return;
    const secondsPerString = noteString.length * secondsPerNote;

    while (nextNoteTime < audioContext.currentTime + secondsPerString) {
      const nodes = scheduleNoteString(
        nextNoteTime,
        secondsPerNote,
        noteString
      );
      connectedNodes.push(nodes);
      nextNoteTime += secondsPerString;
    }

    while (connectedNodes.length > 3) {
      const nodes = connectedNodes.shift();
      for (const node of nodes) {
        node.disconnect(audioOutput);
      }
    }
  }
</script>

<BackgroundTask {onFrame}></BackgroundTask>
