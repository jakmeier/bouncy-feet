import { writable } from 'svelte/store';
import { asset } from '$app/paths';
import { browser } from '$app/environment';
import { audioDelay } from '$lib/stores/System';
import { resolveAudioSchedule } from '$lib/audioTiming.js';

/** @type {AudioContext} */
let audioContext;
const audioBuffers = {};
const audioStore = writable(audioBuffers);
const channels = {};

export async function initAudioContext() {
  if (audioContext) {
    return;
  }
  if (!browser) {
    return;
  }
  audioContext = new AudioContext();
  audioDelay.set((audioContext.outputLatency || 0) * 1000);
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
    audioStore.set(audioBuffers);
    return buffer;
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
  return loadAudio('success', asset('/audio/correct-soft-beep.mp3'));
}

export async function loadBeatSounds() {
  let kickAudioFiles = ['kick', 'kick2'];
  return Promise.all(
    kickAudioFiles.map((name) => loadAudio(name, asset(`/audio/${name}.mp3`)))
  );
}

/** @param {string} name */
export async function loadTrack(name) {
  return loadAudio(name, `https://app.bouncy-feet.ch/media/tracks/${name}.mp3`);
}

/** @param {string} id */
export function playAudio(id) {
  scheduleAudio(id, performance.now());
}

/**
 * @param {string} id
 * @param {number} timestamp in ms as performance timestamp
 * @param {string} channel
 * @param {number} [offsetSeconds] Offset into the decoded buffer.
 * @return {AudioBufferSourceNode}
 */
export function scheduleAudioOnChannel(
  id,
  timestamp,
  channel,
  offsetSeconds = 0
) {
  const { start } = audioSchedule(timestamp);
  return scheduleAudioEx(id, start, channel, offsetSeconds);
}

/**
 * Schedule a continuous track while preserving its wall-clock timeline.
 *
 * A track commonly starts "now". In that case output-latency compensation
 * asks Web Audio to start it in the past, which scheduleAudioEx must clamp to
 * the current audio time. Advance the buffer offset by the clamped duration so
 * future music beats still line up with sounds that could be pre-scheduled.
 *
 * @param {string} id
 * @param {number} timestamp in ms as performance timestamp
 * @param {string} channel
 * @param {number} [offsetSeconds] Offset into the decoded buffer.
 * @return {AudioBufferSourceNode}
 */
export function scheduleAudioTrackOnChannel(
  id,
  timestamp,
  channel,
  offsetSeconds = 0
) {
  const { start, clampedSeconds } = audioSchedule(timestamp);
  return scheduleAudioEx(id, start, channel, offsetSeconds + clampedSeconds);
}

/** @param {number} timestamp */
function audioSchedule(timestamp) {
  const delay = (timestamp - performance.now()) / 1000.0;
  // outputLatency is not supported in all browsers.
  const outputLatency = audioContext.outputLatency || 0;
  return resolveAudioSchedule(audioContext.currentTime, delay, outputLatency);
}

/**
 * @param {string} id
 * @param {number} audioTime in time of audio context
 * @param {string} channel
 * @param {number} [offsetSeconds] Offset into the decoded buffer.
 * @return {AudioBufferSourceNode}
 */
export function scheduleAudioEx(id, audioTime, channel, offsetSeconds = 0) {
  if (!channels[channel]) {
    channels[channel] = new GainNode(audioContext, { gain: 1.0 });
  }

  resumeAudio();
  const source = getAudio(id);
  if (source) {
    source.connect(channels[channel]);
    source.start(
      Math.max(audioContext.currentTime, audioTime),
      Math.max(0, offsetSeconds)
    );
  } else {
    console.warn('no sound buffer for', id);
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
  // Sources scheduled in the future must be explicitly stopped, disconnecting
  // alone leaves them running and can leak stale countdown cues into retries.
  try {
    if (node instanceof AudioBufferSourceNode) node.stop();
  } catch {
    // A source may already have ended or been stopped.
  }
  if (channels[channel]) {
    node.disconnect(channels[channel]);
  }
}

/**
 * @param {string} channel
 * @param {number} gain
 */
export function setChannelGain(channel, gain) {
  if (!audioContext) return;

  resumeAudio();

  if (!channels[channel]) {
    channels[channel] = new GainNode(audioContext, { gain });
    channels[channel].connect(audioContext.destination);
  } else {
    channels[channel].gain.value = gain;
  }
}

// on a page reload, the audio context is usually prevented from starting
// automatically, we have to wait for a user interaction.
function resumeAudio() {
  if (audioContext && audioContext.state === 'suspended') {
    audioContext.resume();
  }
}
