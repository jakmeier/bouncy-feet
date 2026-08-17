export let songs = initSongsLibrary();

/**
 *
 * @param {string} id
 * @param {string} author
 * @param {string} title
 * @param {{drops: SongDrop[]}} timing
 * @returns {Song}
 */
function song(id, author, title, timing) {
  const bpm = parseInt(id.split('bpm')[0], 10);
  return {
    id,
    bpm,
    author,
    title,
    ...timing,
  };
}

function initSongsLibrary() {
  /** @type {Song[]} */
  let library = [
    song('105bpm_tropical_house', 'Aeris', 'Arise', {
      drops: [{ id: 'first', timestampMs: 9_220 }],
    }),
    song('115bpm_house', 'Den Elbriggs', 'Glen Brides', {
      drops: [{ id: 'first', timestampMs: 19_059 }],
    }),
    song('120bpm_tech_house', 'Kryne', 'Kerny', {
      drops: [{ id: 'first', timestampMs: 17_356 }],
    }),
    song('120bpm_groove', 'Iv Eight', 'GROOVE', {
      drops: [{ id: 'first', timestampMs: 32_381 }],
    }), // TODO: update with bought version before calibration
    song(
      '122bpm_you_can_do_this',
      'Alexi Action & Infraction ',
      'You Can Do This',
      {
        drops: [{ id: 'first', timestampMs: 8_571 }],
      }
    ),
  ];
  // TODO: final song names should be checked with authors

  return {
    /**
     * Get a song by its ID.
     * @param {string} id
     * @returns {Song|undefined}
     */
    get: (id) => library.find((s) => s.id === id),
    /**
     * Get all available songs.
     * @returns {Song[]}
     */
    list: () => library,
  };
}
