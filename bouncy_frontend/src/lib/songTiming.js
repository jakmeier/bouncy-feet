// Music plays for 20 beats before the spoken four-beat count-in.
export const DEFAULT_MUSIC_LEAD_IN_BEATS = 20;
export const DEFAULT_COUNT_IN_BEATS = 4;
export const DEFAULT_PLAYBACK_LEAD_IN_BEATS =
  DEFAULT_MUSIC_LEAD_IN_BEATS + DEFAULT_COUNT_IN_BEATS;

/**
 * @param {number} bpm
 * @returns {number}
 */
export function beatDurationMs(bpm) {
  if (!Number.isFinite(bpm) || bpm <= 0) {
    throw new Error(
      `Song BPM must be a positive finite number; received ${bpm}`
    );
  }
  return 60_000 / bpm;
}

/**
 * @param {number} bpm
 * @param {number} [countInBeats]
 * @returns {number}
 */
export function countInDurationMs(bpm, countInBeats = DEFAULT_COUNT_IN_BEATS) {
  if (!Number.isInteger(countInBeats) || countInBeats <= 0) {
    throw new Error(
      `Count-in beats must be a positive integer; received ${countInBeats}`
    );
  }
  return beatDurationMs(bpm) * countInBeats;
}

/**
 * Check song drop metadata before it is used for scheduling.
 *
 * `durationMs` is optional runtime metadata added after decoding the audio track.
 * It is intentionally not required in Songs.js.
 *
 * @param {Song} song
 */
export function validateSongMetadata(song) {
  if (!song || typeof song !== 'object') throw new Error('A song is required');
  beatDurationMs(song.bpm);

  if (!Array.isArray(song.drops) || song.drops.length === 0) {
    throw new Error(`Song ${song.id} must define at least one drop`);
  }

  const ids = new Set();
  let previousTimestampMs = -1;
  for (const drop of song.drops) {
    if (!drop || typeof drop.id !== 'string' || drop.id.trim() === '') {
      throw new Error(`Song ${song.id} has a drop with an empty ID`);
    }
    if (ids.has(drop.id)) {
      throw new Error(`Song ${song.id} has duplicate drop ID "${drop.id}"`);
    }
    ids.add(drop.id);

    const timestampMs = calibratedTimestamp(song.id, drop);
    if (timestampMs <= previousTimestampMs) {
      throw new Error(
        `Song ${song.id} drops must be in strictly increasing timestamp order`
      );
    }
    previousTimestampMs = timestampMs;
  }

  const firstDrop = song.drops.find((drop) => drop.id === 'first');
  if (!firstDrop) throw new Error(`Song ${song.id} must define a "first" drop`);
  calibratedTimestamp(song.id, firstDrop);

  // A first drop may occur before a full count-in fits in the track. In that
  // case playback is delayed and the early count cues run over silence.

  if (
    song.durationMs !== undefined &&
    (!Number.isFinite(song.durationMs) || song.durationMs < 0)
  ) {
    throw new Error(
      `Song ${song.id} durationMs must be a non-negative finite number`
    );
  }
}

/**
 * Amount of silence to leave before starting the audio when a full count-in
 * cannot fit before its selected end marker. Schedule spoken/visual cues at
 * the count-in start and schedule the audio after this delay.
 *
 * @param {Song} song
 * @param {ResolvedSongTiming} timing
 * @returns {number}
 */
export function songPlaybackDelayMs(song, timing) {
  return Math.max(
    0,
    countInDurationMs(song.bpm, timing.countInBeats) -
      (timing.countInEndMs - timing.playbackStartMs)
  );
}

/**
 * @param {Song} song
 * @param {SongTimingOverride} [override]
 * @returns {ResolvedSongTiming}
 */
export function resolveSongTiming(song, override = {}) {
  validateSongMetadata(song);

  if (!override || typeof override !== 'object') {
    throw new Error('Song timing override must be an object');
  }
  if (
    override.countInEndDrop !== undefined &&
    override.countInEndMs !== undefined
  ) {
    throw new Error(
      'Song timing override cannot specify both countInEndDrop and countInEndMs'
    );
  }

  const firstDrop = findDrop(song, 'first');
  let countInEndMs = calibratedTimestamp(song.id, firstDrop);
  if (override.countInEndMs !== undefined) {
    assertTimestamp('countInEndMs', override.countInEndMs);
    countInEndMs = override.countInEndMs;
  } else if (override.countInEndDrop !== undefined) {
    const selectedDrop = findDrop(song, override.countInEndDrop);
    countInEndMs = calibratedTimestamp(song.id, selectedDrop);
  }

  const playbackStartMs =
    override.playbackStartMs === undefined
      ? Math.max(
          0,
          calibratedTimestamp(song.id, firstDrop) -
            countInDurationMs(song.bpm, DEFAULT_PLAYBACK_LEAD_IN_BEATS)
        )
      : override.playbackStartMs;
  assertTimestamp('playbackStartMs', playbackStartMs);

  if (playbackStartMs > countInEndMs) {
    throw new Error('playbackStartMs must not be later than countInEndMs');
  }
  if (
    song.durationMs !== undefined &&
    (playbackStartMs > song.durationMs || countInEndMs > song.durationMs)
  ) {
    throw new Error(
      `Resolved song timing is outside song ${song.id}'s duration`
    );
  }

  return {
    playbackStartMs,
    countInEndMs,
    countInBeats: DEFAULT_COUNT_IN_BEATS,
    musicLeadInMs: Math.max(
      0,
      countInEndMs -
        playbackStartMs -
        countInDurationMs(song.bpm, DEFAULT_COUNT_IN_BEATS)
    ),
  };
}

/** @param {Song} song @param {string} id @returns {SongDrop} */
function findDrop(song, id) {
  const drop = song.drops.find((candidate) => candidate.id === id);
  if (!drop) throw new Error(`Song ${song.id} has no drop named "${id}"`);
  return drop;
}

/** @param {string} songId @param {SongDrop} drop @returns {number} */
function calibratedTimestamp(songId, drop) {
  const { timestampMs } = drop;
  if (
    typeof timestampMs !== 'number' ||
    !Number.isFinite(timestampMs) ||
    timestampMs < 0
  ) {
    throw new Error(
      `Song ${songId} drop "${drop.id}" must have a non-negative finite timestampMs`
    );
  }
  return timestampMs;
}

/** @param {string} name @param {unknown} value */
function assertTimestamp(name, value) {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    throw new Error(`${name} must be a non-negative finite timestamp`);
  }
}
