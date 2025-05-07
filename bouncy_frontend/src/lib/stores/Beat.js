import { Tracker } from '$lib/instructor/bouncy_instructor';
import { dynamicCounter } from '$lib/timer';
import { setContext } from 'svelte';
import { derived, writable } from 'svelte/store';


export const beatStart = writable(performance.now());
export const bpm = writable(132);

export const timeBetweenMoves = derived([bpm], ([$bpm]) => {
    return 30_000 / $bpm;
})

export const beatDuration = derived([bpm], ([$bpm]) => {
    return 60_000 / $bpm;
})

// actually counts subbeat, not full beats
export let beatCounter = dynamicCounter(-1, 1, 100);

/** @type {number | undefined} */
let prevBeatStart = undefined;
const beatZero = derived([timeBetweenMoves, beatStart], ([$timeBetweenMoves, $beatStart]) => {
    // update the beat counter after changes to beatStart
    // (but not when timeBetweenMoves changes)

    // if the beat start becomes negative, animations can use that to start with
    // correct timing
    if (prevBeatStart != $beatStart) {
        prevBeatStart = $beatStart;
        const now = performance.now();
        const currentBeat = (now - $beatStart) / $timeBetweenMoves;
        beatCounter.set(currentBeat);
    }
})
const _dummy_sub = beatZero.subscribe(() => { });


const _sub = timeBetweenMoves.subscribe((value) => beatCounter.setDelay(value));

/** @param {number} value */
export function setBpm(value) {
    bpm.set(value);
}

/** @param {number} value */
export function setBeatStart(value) {
    beatStart.set(value);
}

/** 
 * @param {Tracker} tracker 
 * @returns {()=>void} unsub
 * */
export function registerTracker(tracker) {
    const unsubBpm = bpm.subscribe((value) => {
        tracker.setBpm(value);
    });
    const unsubBeat = beatStart.subscribe((value) => {
        tracker.alignBeat(value);
    });

    setContext('tracker', {
        tracker,
    });

    return () => {
        unsubBpm();
        unsubBeat();
    };
}
