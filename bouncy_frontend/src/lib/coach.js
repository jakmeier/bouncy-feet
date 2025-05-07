// For now, hardcode all coach settings.
// Longterm, it should probably be loaded from RON files.

import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from "./constants";

/**
 * @typedef {Object} Coach 
 * @property {string} name
 * @property {string[]} courseIds
 * @property {AvatarStyleContext} style
 * @property {TranslatedText} description
 * @property {TranslatedText} title
*/
/** @type {Coach[]} */
export const coaches = [
    {
        name: 'juhwang',
        courseIds: ['running-man-basics', 'rm-practice'],
        style: {
            coloring: ORANGE_COLORING,
            bodyShape: {
                strokeWidth: 2,
            },
            headStyle: {
                shape: 'disk',
                size: 1,
                strokeWidth: 1,
            },
        },
        title: {
            de: "Trick-Läufer",
            en: "Trick-Runner"
        },
        description: {
            de: "Lerne den Running Man mit allen Varianten hier bei mir.",
            en: "Learn the Running Man and its variations here with me."
        },
    },

    /** @type {Coach} */
    {
        name: 'chorok',
        courseIds: ['v-step-basics'],
        style: {
            coloring: LEFT_RIGHT_COLORING,
            bodyShape: {
                strokeWidth: 1,
            },
            headStyle: {
                shape: 'circle',
                size: 0.75,
                strokeWidth: 1.5,
            },
        },
        title: {
            de: "Heel-Toe Meister",
            en: "Heel-Toe Master"
        },
        description: {
            de: "Von der Ferse auf die Zehen. Hin und her. Immer wieder, Takt für Takt.",
            en: "From heel to toe. Back and forth. Again and again, beat after beat."
        },
    }
];

