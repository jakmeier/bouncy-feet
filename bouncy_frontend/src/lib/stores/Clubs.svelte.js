/**
 * @typedef {Object} Club
 * @property {number} id
 * @property {string} name
 * @property {string} description
 * @property {AvatarStyleContext} style
 * @property {{ [key: string]: string }} peertubePlaylist
 * @property {object} stats
 *
 * @typedef {Object} ClubsContextData
 * @property {ClubsData} clubsData
 *
 * @typedef {Object} ClubsData
 * @property {Club[]} mine
*/

import { getContext } from "svelte";

/** @returns {ClubsContextData} */
export function getClubsContext() {
    const ctx = getContext('clubs');
    if (!ctx) {
        console.warn("missing clubs context");
    }
    return ctx;
}

/** @type {ClubsData} */
export const clubsData = $state({
    mine: []
    // TODO: consider adding
    // lastUpdated:
})

/**
 * @param {UserContextData} userCtx
 */
export async function loadMyClubs(userCtx) {
    const response = await userCtx.authenticatedGet("/clubs/joined");
    /** @type { {clubs: Club[]} } */
    const data = await response?.json();
    clubsData.mine = [];
    // clubsData.mine.push(...data.clubs);
    for (let club of data.clubs) {
        const defaultValues = mockCourseBase("Default");
        const completeClub = Object.assign(defaultValues, club)
        clubsData.mine.push(completeClub);
    }
    // clubsData.mine.push(...mockData());
}

/**
 * @param {UserContextData} userCtx
 * @param {string} title
 * @param {string} description
 */
export async function createNewClub(userCtx, title, description) {
    if (title.length > 64 || description.length > 1024) {
        // UI should catch this!
        throw new Error("club title or description too long");
    }

    const response = await userCtx.authenticatedPost("/clubs/create", { title, description });

    if (response?.status !== 201) {
        console.error("Failed to create club", response);
        return;
    }

    /** @type { Club } */
    const club = await response?.json();
    const defaultValues = mockCourseBase("Default");
    const completeClub = Object.assign(defaultValues, club)
    clubsData.mine.push(completeClub);
}

/** @returns {Club[]} */
function mockData() {
    return [
        mockCourseBase("East Attitude Shufflers"),
        mockCourseBase("SG Shufflers", '#019934', '#ffffff'),
        mockCourseBase("SWISS Shufflers", '#FF0000', '#ffffff') // alt: #DA291C
    ];
}

/**
 * @param {string | undefined} name
 * @param {string | undefined} [mainColor]
 * @param {string | undefined} [secondaryColor]
 *
 * @returns {Club}
 */
function mockCourseBase(name, mainColor, secondaryColor) {
    return {
        id: 0,
        name: name || "Mock Club",
        description: "This is a Mock Club",
        // courseIds: ['running-man-basics', 'rm-practice'],
        style: {
            coloring: {
                leftColor: mainColor || 'var(--avatar-left)',
                rightColor: secondaryColor || 'var(--avatar-right)',
                headColor: mainColor || 'var(--avatar-head)',
            },
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
        stats: {
            members: 0,
        },
        peertubePlaylist: {}
    };
}