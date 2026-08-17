import { describe, expect, it } from 'vitest';
import { resolveAudioSchedule } from './audioTiming.js';

describe('audio scheduling', () => {
  it('pre-schedules future audio by the output latency', () => {
    expect(resolveAudioSchedule(10, 2, 0.2)).toEqual({
      start: 11.8,
      clampedSeconds: 0,
    });
  });

  it('reports the part of latency compensation that cannot be scheduled', () => {
    const schedule = resolveAudioSchedule(10, 0, 0.2);
    expect(schedule.start).toBe(10);
    expect(schedule.clampedSeconds).toBeCloseTo(0.2);
  });

  it('also includes a late call in the clamped duration', () => {
    expect(resolveAudioSchedule(10, -0.05, 0.2)).toEqual({
      start: 10,
      clampedSeconds: 0.25,
    });
  });
});
