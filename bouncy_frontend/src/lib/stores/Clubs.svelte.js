/**
 * @typedef {Object} Club
 * @property {string} name
 * @property {string} description
 * @property {AvatarStyleContext} style
 * @property {{ [key: string]: string }} peertubePlaylist
 * @property {object} stats
*/

export const clubsData = $state({
    /** @type {Club[]} */
    mine: []

    // TODO: consider adding
    // lastUpdated: 
})

export async function loadMyClubs() {
    // TODO: load from API
    // Idee: EA Shufflers zuerst, später ev SG Shuffler, oder direkt Swiss Shufflers?
    clubsData.mine = [];
    clubsData.mine.push(...mockData());
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
            members: 77,
        },
        peertubePlaylist: {}
    };
}