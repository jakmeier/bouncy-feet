export let songs = initSongsLibrary();

/** @param {string} id 
 * 
 * @returns {Song} 
*/
function song(id, author, title) {
    const bpm = parseInt(id.split("bpm")[0], 10);
    return {
        id,
        bpm,
        author,
        title,
    };
}

function initSongsLibrary() {

    /** @type {Song[]} */
    let library = [
        song("105bpm_tropical_house", "Aeris", "Arise"),
        song("115bpm_house", "Den Elbriggs", "Glen Brides"),
        song("120bpm_tech_house", "Kryne", "Kerny"),
        song("122bpm_you_can_do_this", "Alexi Action & Infraction ", "You Can Do This"),
    ];
    // TODO: final song names should be checked with authors

    return {
        /**
         * Get a song by its ID.
         * @param {string} id
         * @returns {Song|undefined}
         */
        get: (id) => library.find(s => s.id === id),
        /**
         * Get all available songs.
         * @returns {Song[]}
         */
        list: () => library,
    };
}