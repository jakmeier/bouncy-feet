import { Tracker } from '$lib/instructor/bouncy_instructor';
import { setContext } from 'svelte';
import { derived, writable } from 'svelte/store';


export const beatStart = writable(Date.now());
export const bpm = writable(132);
export const halfSpeed = writable(false);

export const timeBetweenMoves = derived([bpm, halfSpeed], ([$bpm, $halfSpeed]) => {
    return ($halfSpeed ? 60_000 : 30_000) / $bpm;
})

/** @param {number} value */
export function setBpm(value) {
    bpm.set(value);
}

/** @param {number} value */
export function setBeatStart(value) {
    beatStart.set(value);
}

/** @param {boolean} yes */
export function setHalfSpeed(yes) {
    halfSpeed.set(yes);
}

/** @param {Tracker} tracker */
export function registerTracker(tracker) {
    bpm.subscribe((value) => {
        tracker.setBpm(value);
    });
    beatStart.subscribe((value) => {
        tracker.alignBeat(BigInt(value));
    });

    setContext('tracker', {
        tracker,
    })
}