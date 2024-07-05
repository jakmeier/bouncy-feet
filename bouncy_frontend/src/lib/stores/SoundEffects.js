
import { writable, get } from 'svelte/store';
import { base } from '$app/paths';

const audioContext = new AudioContext();
const soundEffects = writable({ success: undefined });

async function initAudioContext() {
  const sounds = {
    success: await loadSound(`${base}/audio/correct-soft-beep.mp3`),
  };

  soundEffects.set(sounds);
}

/** @param {string | URL | Request} url */
async function loadSound(url) {
  const response = await fetch(url);
  const arrayBuffer = await response.arrayBuffer();
  return audioContext.decodeAudioData(arrayBuffer);
}

export function playSuccessSound() {
  if (get(soundEffects).success) {
    const source = audioContext.createBufferSource();
    source.buffer = get(soundEffects).success;
    source.connect(audioContext.destination);
    source.start(0);
  } else {
    console.warn("sound not loaded");
  }
}

await initAudioContext();
