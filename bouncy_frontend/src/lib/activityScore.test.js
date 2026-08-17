import { describe, expect, it } from 'vitest';
import { detectionScore } from './activityScore.js';

const detection = {
  poseMatches: 7,
  poseMisses: 2,
  stepMatches: 2,
  stepMisses: 1,
};

describe('detectionScore', () => {
  it('keeps pose scoring as the default', () => {
    expect(detectionScore(detection)).toEqual({ hits: 7, misses: 2 });
  });

  it('uses full repetitions in step mode', () => {
    expect(detectionScore(detection, 'steps')).toEqual({ hits: 2, misses: 1 });
  });
});
