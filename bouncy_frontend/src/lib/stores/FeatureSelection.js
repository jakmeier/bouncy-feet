

import { derived, readable } from "svelte/store";
import { dev as envDev, browser } from '$app/environment';

let privDev = envDev;
export const dev = readable(privDev, (set) => {
    if (browser) {
        // @ts-ignore
        window.toggleDev = () => { privDev = !privDev; set(privDev); };
    }
});
export const version = derived(dev, ($dev) => $dev ? 0.999 : 0.004);

/** @type {import("svelte/motion").Readable<Features>} */
export const features = derived([version, dev], ([$v, $dev]) => {
    return {
        /* Fully enabled features for now but might be disabled again*/
        enableDanceCollection: $v >= 0.003,
        enableDanceCreator: $v >= 0.003,

        /* Partially enabled features */
        enableStepRecording: (stepName) => STABLE_TRACKING_STEPS.includes(stepName),

        /* Features that are not ready to be released */
        enableAvatarRotation: $v >= 0.999,
        enableFreestyleRecording: $v >= 0.999,
        enableCourses: $v >= 0.005,

        /* Features that stay in dev */
        enableDevView: $dev,
    }
}
);

/** Steps that should be possible to track, with passing tests. */
export const STABLE_TRACKING_STEPS = [
    "Running Man",
    "Gangsta Hop",
    "Kicking Reverse RM",
    "Reverse RM",
    "Happy Feet"
];
