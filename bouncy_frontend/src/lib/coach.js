// For now, hardcode all coach settings.
// Longterm, it should probably be loaded from RON files.

import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from "./constants";

/**
 * @typedef {Object} Coach 
 * @property {string} name
 * @property {string[]} courseIds
 * @property {Object} steps
 * @property {AvatarStyleContext} style
 * @property {TranslatedText} description
 * @property {TranslatedText} title
 * @property {{ [key: string]: string }} peertubePlaylist
*/
/** @type {Coach[]} */
export const coaches = [
    {
        name: 'rob',
        steps: {
            "rm-0": {
                video: "https://app.bouncy-feet.ch/media/videos/steps/running_man.mp4",
                courses: ['running-man-basics', 'rm-practice']
            },
            "reverse-rm-0": {
                video: "https://app.bouncy-feet.ch/media/videos/steps/reverse-rm.mp4",
                // TODO add courses
                courses: ["rrm-basics"]
            },
            "pp-0": {
                video: "https://app.bouncy-feet.ch/media/videos/steps/drm.mp4",
                courses: []
            }
        },
        courseIds: ['running-man-basics', 'rm-practice'],
        style: {
            coloring: ORANGE_COLORING,
            bodyShape: {
                strokeWidth: 1,
            },
            headStyle: {
                shape: 'disk',
                size: 0.75,
                strokeWidth: 1,
            },
            pageColoring: {
                pageColor: "var(--theme-accent)",
                secondaryColor: "var(--theme-accent-medium)",
                danceFloorColor: "var(--theme-neutral-light)",
                fontColor: "var(--theme-neutral-black)",
            }
        },
        title: {
            de: "Rave Energie",
            en: "Rave Energy"
        },
        description: {
            de: "Tritt ein in das verlassene Warenhaus von Rob und lerne alles über den **Running Man**.",
            en: "Enter Rob's abandoned warehouse to learn about the **Running Man** and its variations."
        },
        peertubePlaylist: {
            dev: "3FWFPJds8qwVJUbzSsjFzH",
            prod: "",
        }
    },

    /** @type {Coach} */
    {
        name: 'charles',
        steps: {
            "t-0": {
                // TODO(July): add video and course
                video: "",
                courses: []
            },
            "v-0": {
                // TODO(July): add video and course (maybe charleston / happy feet instead)
                video: "",
                courses: []
            }
        },
        courseIds: ['v-step-basics'],
        style: {
            coloring: LEFT_RIGHT_COLORING,
            bodyShape: {
                strokeWidth: 1,
            },
            headStyle: {
                // TODO: try tophat
                shape: 'disk',
                size: 0.75,
                strokeWidth: 1,
            },
            pageColoring: {
                pageColor: "var(--theme-main-alt)",
                secondaryColor: "var(--theme-main)",
                danceFloorColor: "var(--theme-neutral-light)",
                fontColor: "var(--theme-neutral-black)",
            }
        },
        title: {
            de: "Pure Freude",
            en: "Pure Joy"
        },
        description: {
            de: "Willkommen in den Hallen von Charles. Komm und tanze die **Happy Feets** und den **Charleston** mit uns!",
            en: "Welcome to Charles' dancing halls, where **Happy Feets** and **Charleston** rule."
        },
        peertubePlaylist: {
            dev: "nAkAtJ2sdYFN73HZEjDft2",
            prod: "",
        }
    }
];

