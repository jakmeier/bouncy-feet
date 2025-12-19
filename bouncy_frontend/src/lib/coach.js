// For now, hardcode all coach settings.
// Longterm, it should probably be loaded from RON files.

import { LEFT_RIGHT_COLORING, ORANGE_COLORING, TEAL_COLORING } from "./constants";

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
                fontOnDanceFloorColor: "var(--theme-neutral-black)",
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
            localDev: "9aBHGBL88zSb6u1esWzDgD",
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
                fontOnDanceFloorColor: "var(--theme-neutral-black)",
                fontColor: "var(--theme-neutral-black)",
            }
        },
        title: {
            de: "Pure Freude",
            en: "Pure Joy"
        },
        description: {
            de: "Willkommen in den Hallen von Charles. Komm und tanze die **Happy Feets** und den **Charleston** mit uns!",
            en: "Welcome to Charles' dancing halls, where **Happy Feets** and the **Charleston** rule."
        },
        peertubePlaylist: {
            localDev: "9aBHGBL88zSb6u1esWzDgD",
            dev: "nAkAtJ2sdYFN73HZEjDft2",
            prod: "",
        }
    },

    /** @type {Coach} */
    {
        name: 'frank',
        steps: {
            // TODO: add dnb steps
            "back-and-forth-right": {
                // TODO: add better video and course
                video: "https://app.bouncy-feet.ch/media/videos/c6/tutorial.mp4",
                courses: ['dnb-basics']
            },
        },
        // TODO: add courses
        courseIds: ['dnb-basics'],
        style: {
            coloring: TEAL_COLORING,
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
                pageColor: "var(--theme-teal)",
                secondaryColor: "var(--theme-teal-dark)",
                danceFloorColor: "var(--theme-neutral-almost-black)",
                fontOnDanceFloorColor: "var(--theme-teal)",
                fontColor: "var(--theme-neutral-black)",
            }
        },
        title: {
            de: "Schnelle Rhythmen",
            en: "Fast Rhythms"
        },
        description: {
            de: "Bum-bum Tz! Hier wird zu Break-Beats auf über 170 bpm getanzt.",
            en: "Boom-boom Tss! Here we dance to break-beats above 170 bpm."
        },
        peertubePlaylist: {
            localDev: "9aBHGBL88zSb6u1esWzDgD",
            dev: "hLhCbsRyaTBgQYcJFW2XbQ",
            prod: "",
        }
    }
];

/**
 * @param {string} coachId
 */
export function coachData(coachId) {
    const coachData = coaches.find((c) => c.name === coachId);
    if (coachData) {
        return coachData;
    } else {
        return coaches[0];
    }
}
