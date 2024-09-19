import { writable } from "svelte/store";


export const audioDelay = writable(0);

// Time that passes between when a video frame was recorded until the mediapipe
// output is available.
export const mediapipeDelayLastValue = writable(0);
export const mediapipeDelayTotal = writable(0);
export const mediapipeDelayNum = writable(0);

// Delay of calling the `trackFrame` function, which causes a blocking delay on
// the main thread.
export const trackSyncDelayLastValue = writable(0);
export const trackSyncDelayTotal = writable(0);
export const trackSyncDelayNum = writable(0);

// Time spent on evaluating a dance in our own (WASM) code.
export const detectionDelayLastValue = writable(0);
export const detectionDelayTotal = writable(0);
export const detectionDelayNum = writable(0);

export function recordMediaPipeDelay(ms) {
    mediapipeDelayLastValue.set(ms);
    mediapipeDelayTotal.update((prev) => prev + ms);
    mediapipeDelayNum.update((prev) => prev + 1);
}

export function recordTrackSyncDelay(ms) {
    trackSyncDelayLastValue.set(ms);
    trackSyncDelayTotal.update((prev) => prev + ms);
    trackSyncDelayNum.update((prev) => prev + 1);
}

export function recordDetectionDelay(ms) {
    detectionDelayLastValue.set(ms);
    detectionDelayTotal.update((prev) => prev + ms);
    detectionDelayNum.update((prev) => prev + 1);
}

export function resetSystemStats() {
    mediapipeDelayLastValue.set(0);
    mediapipeDelayTotal.set(0);
    mediapipeDelayNum.set(0);

    trackSyncDelayLastValue.set(0);
    trackSyncDelayTotal.set(0);
    trackSyncDelayNum.set(0);

    detectionDelayLastValue.set(0);
    detectionDelayTotal.set(0);
    detectionDelayNum.set(0);
}