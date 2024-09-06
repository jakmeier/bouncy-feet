import { Tracker } from '$lib/instructor/bouncy_instructor';
import { setContext } from 'svelte';
import { derived, readable } from 'svelte/store';

let privateSetBeatStart = (/** @type {number} */ num) => { };
let privateSetBpm = (/** @type {number} */ num) => { };
let privateSetHalfSpeed = (/** @type {boolean} */ yes) => { };

export const beatStart = readable(Date.now(), (set) => {
    privateSetBeatStart = set;
});

export const bpm = readable(132, (set) => {
    privateSetBpm = set;
});

export const halfSpeed = readable(false, (set) => {
    privateSetHalfSpeed = set;
});

export const timeBetweenMoves = derived([bpm, halfSpeed], ([$bpm, $halfSpeed]) => {
    return ($halfSpeed ? 60_000 : 30_000) / $bpm;
})

/** @param {number} bpm */
export function setBpm(bpm) {
    privateSetBpm(bpm)
}

/** @param {number} beatStart */
export function setBeatStart(beatStart) {
    privateSetBeatStart(beatStart)
}

/** @param {boolean} yes */
export function setHalfSpeed(yes) {
    privateSetHalfSpeed(yes)
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