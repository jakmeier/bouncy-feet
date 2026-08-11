/**
 * Challenge attempts have this minimum duration, unless it takes the dancer
 * longer to get the required score. */
export const MINIMUM_DURATION_MS = 20_000;
export const REQUIRED_HIT_RATIO = 0.6;

/** @param {number} opportunities */
export function requiredHits(opportunities) {
  return Math.ceil(opportunities * REQUIRED_HIT_RATIO);
}

/**
 * Decide whether an attempt has earned completion. `challengeDurationMs` is
 * supplied by the challenge definition and is rounded up to a full pattern.
 *
 * @param {{ elapsedMs: number, hits: number, challengeDurationMs: number, requiredHits: number }} attempt
 */
export function hasCompletedChallengeAttempt({
  elapsedMs,
  hits,
  challengeDurationMs,
  requiredHits: minimumHits,
}) {
  return elapsedMs >= challengeDurationMs && hits >= minimumHits;
}

/**
 * A playable song must last through the initial score window.
 *
 * @param {number} songDurationMs
 * @param {number} challengeDurationMs
 */
export function hasPlayableChallengeSong(songDurationMs, challengeDurationMs) {
  return (
    Number.isFinite(songDurationMs) &&
    songDurationMs >= MINIMUM_DURATION_MS &&
    songDurationMs >= challengeDurationMs
  );
}
