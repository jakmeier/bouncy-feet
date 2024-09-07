
import { writable, get } from 'svelte/store';
import { base } from '$app/paths';
import { browser } from '$app/environment';

/** @type {AudioContext} */
export let audioContext;
const audioBuffers = {};
const audioStore = writable(audioBuffers);
const channels = {};

async function initAudioContext() {
  if (audioContext) {
    return;
  }
  if (!browser) {
    return;
  }
  audioContext = new AudioContext();
  // @ts-ignore
  channels.default = new GainNode(audioContext, { gain: 1.0 });
  // @ts-ignore
  channels.default.connect(audioContext.destination);
}

/** 
 * @param {string | URL | Request} url
 * @returns {Promise<AudioBuffer>}
 * */
async function loadSoundFile(url) {
  const response = await fetch(url);
  const arrayBuffer = await response.arrayBuffer();
  return audioContext.decodeAudioData(arrayBuffer);
}

/**
 * @param {string | URL | Request} url
 * @param {string} id
 */
export async function loadAudio(id, url) {
  try {
    const buffer = await loadSoundFile(url);
    audioBuffers[id] = buffer;
    audioStore.set(audioBuffers)
  } catch (err) {
    console.error(`Error loading audio from ${url}:`, err);
  }
}

/**
 * @param {string} id
 * @return {AudioBufferSourceNode | undefined}
 */
export function getAudio(id) {
  if (audioBuffers[id]) {
    const source = audioContext.createBufferSource();
    source.buffer = audioBuffers[id];
    return source;
  } else {
    console.warn(`sound ${id} not loaded`);
  }
}

export function getAudioContext() {
  return audioContext;
}

export async function loadSuccessSound() {
  return loadAudio('success', `${base}/audio/correct-soft-beep.mp3`);
}

export async function loadBeatSounds() {
  let kickAudioFiles = ['kick', 'kick2'];
  return Promise.all(kickAudioFiles.map((name) => loadAudio(name, `${base}/audio/${name}.mp3`)));
}

/** @param {string} id */
export function playAudio(id) {
  scheduleAudio(id, performance.now());
}


/** 
 * @param {string} id 
 * @param {number} timestamp in ms as performance timestamp 
 * @param {string} channel
 * @return {AudioBufferSourceNode}
*/
export function scheduleAudioOnChannel(id, timestamp, channel) {
  const delay = (timestamp - performance.now()) / 1000.0;
  const start = audioContext.currentTime + delay - audioContext.outputLatency;
  return scheduleAudioEx(id, start, channel);
}

/** 
 * @param {string} id 
 * @param {number} audioTime in time of audio context
 * @param {string} channel
 * @return {AudioBufferSourceNode}
*/
export function scheduleAudioEx(id, audioTime, channel) {
  if (!channels[channel]) {
    channels[channel] = new GainNode(audioContext, { gain: 1.0 });
  }

  if (audioContext.state === 'suspended') {
    // on a page reload, the audio context is usually prevented from starting
    // automatically, we have to wait for a user interaction.
    audioContext.resume();
  }
  const source = getAudio(id);
  if (source) {
    source.connect(channels[channel]);
    source.start(Math.max(0, audioTime));
  } else {
    console.warn("no sound buffer for", id);
  }
  return source;
}

/** 
 * @param {string} id 
 * @param {number} timestamp in ms as performance timestamp 
 * @return {AudioBufferSourceNode}
*/
export function scheduleAudio(id, timestamp) {
  return scheduleAudioOnChannel(id, timestamp, 'default');
}

/** 
 * @param {AudioNode} node
 * @param {string} channel
*/
export function cleanupAudioNode(node, channel) {
  if (channels[channel]) {
    node.disconnect(channels[channel]);
  }
}

/** 
 * @param {string} channel
 * @param {number} gain
*/
export function setChannelGain(channel, gain) {
  if (!channels[channel]) {
    channels[channel] = new GainNode(audioContext, { gain });
    channels[channel].connect(audioContext.destination);
  } else {
    channels[channel].gain.value = gain;
  }
}

await initAudioContext();
