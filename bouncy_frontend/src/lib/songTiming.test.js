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

  it('uses the first drop and an eight-beat lead-in by default', () => {
    expect(resolveSongTiming(song)).toEqual({
      playbackStartMs: 6_000,
      countInEndMs: 10_000,
      countInBeats: 4,
      musicLeadInMs: 2_000,
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
    const earlyDropSong = { ...song, drops: [{ id: 'first', timestampMs: 3_000 }] };
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
