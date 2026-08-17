import { describe, expect, it } from 'vitest';
import {
  beatDurationMs,
  countInDurationMs,
  resolveSongTiming,
  songPlaybackDelayMs,
  validateSongMetadata,
} from './songTiming.js';

const song = {
  id: '120bpm_test',
  author: 'Test author',
  title: 'Test title',
  bpm: 120,
  drops: [
    { id: 'first', timestampMs: 10_000 },
    { id: 'second', timestampMs: 42_000 },
  ],
};

describe('song timing', () => {
  it('calculates beat and count-in durations from BPM', () => {
    expect(beatDurationMs(120)).toBe(500);
    expect(countInDurationMs(120)).toBe(2_000);
    expect(countInDurationMs(105)).toBeCloseTo(2_285.714286);
  });

  it('plays 20 music beats before the four-beat count-in', () => {
    const lateDropSong = {
      ...song,
      drops: [{ id: 'first', timestampMs: 15_000 }],
    };
    expect(resolveSongTiming(lateDropSong)).toEqual({
      playbackStartMs: 3_000,
      countInEndMs: 15_000,
      countInBeats: 4,
      musicLeadInMs: 10_000,
    });
  });

  it('starts at the beginning when fewer than 24 beats precede the drop', () => {
    const shortIntroSong = {
      ...song,
      drops: [{ id: 'first', timestampMs: 11_500 }],
    };
    expect(resolveSongTiming(shortIntroSong)).toEqual({
      playbackStartMs: 0,
      countInEndMs: 11_500,
      countInBeats: 4,
      musicLeadInMs: 9_500,
    });
  });

  it('rejects invalid or conflicting configuration', () => {
    expect(() =>
      resolveSongTiming(song, { countInEndDrop: 'missing' })
    ).toThrow('no drop named');
    expect(() =>
      resolveSongTiming(song, { countInEndDrop: 'first', countInEndMs: 10_000 })
    ).toThrow('cannot specify both');
    expect(() => resolveSongTiming(song, { playbackStartMs: 20_000 })).toThrow(
      'must not be later'
    );
  });

  it('delays playback over silence when the first drop has too little lead-in', () => {
    const earlyDropSong = {
      ...song,
      drops: [{ id: 'first', timestampMs: 3_000 }],
    };
    const timing = resolveSongTiming(earlyDropSong);
    expect(timing).toEqual({
      playbackStartMs: 0,
      countInEndMs: 3_000,
      countInBeats: 4,
      musicLeadInMs: 1_000,
    });
    expect(songPlaybackDelayMs(earlyDropSong, timing)).toBe(0);
  });

  it('rejects invalid drop metadata and timings beyond a known duration', () => {
    expect(() =>
      validateSongMetadata({
        ...song,
        drops: [
          { id: 'first', timestampMs: 10_000 },
          { id: 'again', timestampMs: 10_000 },
        ],
      })
    ).toThrow('strictly increasing');
    expect(() => resolveSongTiming({ ...song, durationMs: 9_000 })).toThrow(
      'outside'
    );
  });
});
