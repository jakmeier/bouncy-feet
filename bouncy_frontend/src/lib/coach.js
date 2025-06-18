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
*/
/** @type {Coach[]} */
export const coaches = [
    {
        name: 'rob',
        steps: {
            "rm-0": {
                // TODO: use correct video
                video: "https://app.bouncy-feet.ch/media/videos/steps/reverse-rm.mp4",
                courses: ['running-man-basics', 'rm-practice']
            },
            "reverse-rm-0": {
                video: "https://app.bouncy-feet.ch/media/videos/steps/reverse-rm.mp4",
                // TODO add courses
                courses: ["rrm-basics"]
            },
            "pp-0": {
                video: "",
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
            de: "Rave",
            en: "Rave"
        },
        description: {
            de: "Tritt ein in das verlassene Warenhaus von Rob und lerne alles über den **Running Man**.",
            en: "Enter Rob's abandoned warehouse to learn about the **Running Man** and its variations."
        },
    },

    /** @type {Coach} */
    {
        name: 'charles',
        steps: {
            "t-0": {
                video: "",
                courses: []
            }, "v-0": {
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
            de: "Freude",
            en: "Joyful"
        },
        description: {
            de: "Willkommen in den Hallen von Charles. Komm und tanze die **Happy Feets** und den **Charleston** mit uns!",
            en: "Welcome to Charles' dancing halls, where **Happy Feets** and **Charleston** rule."
        },
    }
];

