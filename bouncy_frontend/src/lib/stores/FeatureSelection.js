

import { derived, readable, writable } from "svelte/store";
import { dev as envDev, browser } from '$app/environment';

let experimental = writable(false);
let privDev = false;
export const dev = readable(privDev, (set) => {
    if (browser) {
        // @ts-ignore
        window.toggleDev = () => { privDev = !privDev; set(privDev); };
    }
});
export const displayedVersion = writable(0.006000);
export const version = derived([dev, displayedVersion], ([$dev, $version]) => $dev ? 0.999 : $version);
export const versionString = derived([dev, displayedVersion], ([$dev, $version]) => ($dev ? "DEV " : "") + versionNumberToString($version));

function versionNumberToString(v) {
    let prefix = "alpha ";
    let major = 0;
    let minor = Math.floor(v * 1000);
    let patch = Math.floor(v * 1000000) % 1000;
    return `${prefix}${major}.${minor}.${patch}`;
}

/** @type {import("svelte/motion").Readable<Features>} */
export const features = derived([version, dev, experimental], ([$v, $dev, $experimental]) => {
    return {
        /* Fully enabled features for now but might be disabled again*/
        enableDanceCollection: $v >= 0.003,
        enableDanceCreator: $v >= 0.003,
        enableCourses: $v >= 0.005,

        /* Partially enabled features */
        enableStepRecording: (stepName) => STABLE_TRACKING_STEPS.includes(stepName),
        enableEditorPage: $experimental,

        /* Features that are not ready to be released */
        enableAvatarRotation: $v >= 0.999,
        enableFreestyleRecording: $v >= 0.999,

        /* Features that stay in dev */
        enableDevView: $dev,
    }
}
);

/** @param {boolean} yes */
export function showExperimentalFeatures(yes) {
    experimental.set(yes);
}

/** Steps that should be possible to track, with passing tests. */
export const STABLE_TRACKING_STEPS = [
    "Running Man",
    "Gangsta Hop",
    "Kicking Reverse RM",
    "Reverse RM",
    "Happy Feet"
];
