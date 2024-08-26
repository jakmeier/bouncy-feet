
import { writable, get } from 'svelte/store';
import { base } from '$app/paths';
import { browser } from '$app/environment';

/** @type {AudioContext} */
export let audioContext;
const audioBuffers = {};
const audioStore = writable(audioBuffers);

async function initAudioContext() {
  if (!browser) {
    return;
  }
  audioContext = new AudioContext();
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

export async function loadSuccessSound() {
  return loadAudio('success', `${base}/audio/correct-soft-beep.mp3`);
}

export function playSuccessSound() {
  playAudio('success');
}

/** @param {string} id */
export function playAudio(id) {
  scheduleAudio(id, Date.now())
}

/** 
 * @param {string} id 
 * @param {number} timestamp in ms as UNIX timestamp 
*/
export function scheduleAudio(id, timestamp) {
  if (audioContext.state === 'suspended') {
    // on a page reload, the audio context is usually prevented from starting
    // automatically, we have to wait for a user interaction.
    audioContext.resume();
  }
  const source = getAudio(id);
  if (source) {
    source.connect(audioContext.destination);
    const audioOffset = Date.now() / 1000.0 - audioContext.currentTime;
    source.start(timestamp / 1000.0 - audioOffset);
  } else {
    console.warn("no sound buffer for", id);
  }
}

await initAudioContext();
