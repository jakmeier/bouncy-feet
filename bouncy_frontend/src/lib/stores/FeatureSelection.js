

import { derived, readable } from "svelte/store";
import { dev as envDev, browser } from '$app/environment';

let privDev = envDev;
export const dev = readable(privDev, (set) => {
    if (browser) {
        // @ts-ignore
        window.toggleDev = () => { privDev = !privDev; set(privDev); };
    }
});
export const version = derived(dev, ($dev) => $dev ? 0.999 : 0.002);

/** @type {import("svelte/motion").Readable<Features>} */
export const features = derived([version, dev], ([$v, $dev]) => {
    return {
        /* Features that are not ready to be released */
        enableAvatarRotation: $v >= 0.999,
        enableFreestyleRecording: $v >= 0.999,
        enableDanceCollection: $v >= 0.003,
        enableDanceCreator: $v >= 0.003,

        /* Features that stay in dev */
        enableDevView: $dev,
    }
}
);
