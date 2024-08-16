<script>
  import { base } from '$app/paths';
  import { onDestroy, onMount } from 'svelte';
  import BackgroundTask from './BackgroundTask.svelte';

  export let bpm = 120;
  export let isOn = false;

  let initialized = false;
  $: initialized && (isOn ? startAudio() : stopAudio());
  $: initialized && bpm && resetAudio();

  let countAudioFiles = ['one.mp3', 'two.mp3', 'three.mp3', 'four.mp3'];
  let andAudioFiles = ['and_0.mp3', 'and_1.mp3', 'and_2.mp3'];
  let kickAudioFiles = ['kick.mp3', 'kick2.mp3'];
  /** @type {AudioContext} */
  let audioContext;
  /** @type {GainNode} */
  let audioOutput;
  /** @type {AudioBuffer[]} */
  let countsSounds = [];
  /** @type {AudioBuffer[]} */
  let andSounds = [];
  /** @type {AudioBuffer[]} */
  let kickSounds = [];
  /** @type {number} seconds in audio context, when the first unscheduled note
   * should be scheduled */
  let nextNoteTime = 0.0;
  let halfBeat = 0;
  let isPlaying = false;
  let slowNoteString = '    1 2 12341234';
  let fastNoteString = '    1 a 2 a 1a2a3a4a1a2a3a4a';
  $: noteString = bpm > 109 ? fastNoteString : slowNoteString;
  /**
   * batches of connected audio nodes that should be disconnected at some point
   * @type {AudioBufferSourceNode[][]}
   */
  let connectedNodes = [];

  onMount(async () => {
    audioContext = new AudioContext();
    audioOutput = new GainNode(audioContext);
    countsSounds = await Promise.all(countAudioFiles.map(loadAudioSource));
    andSounds = await Promise.all(andAudioFiles.map(loadAudioSource));
    kickSounds = await Promise.all(kickAudioFiles.map(loadAudioSource));

    if (isOn) startAudio();
    nextNoteTime = audioContext.currentTime;
    initialized = true;
  });

  onDestroy(() => {
    stopAudio();
    audioContext.close();
  });

  /**
   * @param {any} filename
   * @returns {Promise<AudioBuffer>}
   */
  async function loadAudioSource(filename) {
    const url = `${base}/audio/${filename}`;
    const response = await fetch(url);
    const arrayBuffer = await response.arrayBuffer();
    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
    return audioBuffer;
  }

  /**
   * @param {number} time
   * @param {AudioBuffer} buffer
   * @return {AudioBufferSourceNode}
   */
  function scheduleNote(time, buffer) {
    const bufferSource = audioContext.createBufferSource();
    bufferSource.buffer = buffer;
    bufferSource.connect(audioOutput);
    bufferSource.start(time);
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
      const count = noteBuffer(note);
      if (count) {
        const node = scheduleNote(time, count);
        nodes.push(node);
      }
      // also add a kick
      const node = scheduleNote(time, kickSounds[halfBeat % 2]);
      nodes.push(node);
      time += beatDuration;
      halfBeat++;
    }
    return nodes;
  }

  let andCounter = 0;
  /**
   * @param {string} note
   * @returns {AudioBuffer|undefined}
   */
  function noteBuffer(note) {
    if (note >= '0' && note <= '9') {
      return countsSounds[parseInt(note) - 1];
    }
    if (note === 'a') {
      andCounter++;
      return andSounds[(andCounter - 1) % andSounds.length];
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
    const secondsPerNote = 30.0 / bpm;
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
