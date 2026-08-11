import { describe, expect, it } from 'vitest';
import {
  hasCompletedChallengeAttempt,
  hasPlayableChallengeSong,
  MINIMUM_DURATION_MS,
  requiredHits,
} from './challengeAttempt.js';

describe('challenge attempt rules', () => {
  it('rounds the 60% hit threshold up', () => {
    expect(requiredHits(60)).toBe(36);
    expect(requiredHits(56)).toBe(34);
  });

  it('does not complete before the full initial duration', () => {
    expect(
      hasCompletedChallengeAttempt({
        elapsedMs: 31_999,
        hits: 34,
        challengeDurationMs: 32_000,
        requiredHits: 34,
      })
    ).toBe(false);
  });

  it('allows recovery after misses once enough hits are earned', () => {
    expect(
      hasCompletedChallengeAttempt({
        elapsedMs: 32_000,
        hits: 33,
        challengeDurationMs: 32_000,
        requiredHits: 34,
      })
    ).toBe(false);
    expect(
      hasCompletedChallengeAttempt({
        elapsedMs: 33_000,
        hits: 34,
        challengeDurationMs: 32_000,
        requiredHits: 34,
      })
    ).toBe(true);
  });

  it('rejects a song too short for the initial score window', () => {
    expect(
      hasPlayableChallengeSong(MINIMUM_DURATION_MS - 1, MINIMUM_DURATION_MS - 1)
    ).toBe(false);
    expect(
      hasPlayableChallengeSong(MINIMUM_DURATION_MS, MINIMUM_DURATION_MS)
    ).toBe(true);
    expect(
      hasPlayableChallengeSong(MINIMUM_DURATION_MS, MINIMUM_DURATION_MS + 1)
    ).toBe(false);
  });
});
