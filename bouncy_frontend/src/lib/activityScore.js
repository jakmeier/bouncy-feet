/**
 * Return the score counters for a detection result.
 *
 * Pose scoring remains the default for existing activities. Challenges opt in
 * to step scoring, where a hit means every pose in the repetition matched.
 *
 * @param {{ poseMatches: number, poseMisses: number, stepMatches: number, stepMisses: number }} detection
 * @param {'poses' | 'steps'} [mode]
 */
export function detectionScore(detection, mode = 'poses') {
  if (mode === 'steps') {
    return { hits: detection.stepMatches, misses: detection.stepMisses };
  }
  return { hits: detection.poseMatches, misses: detection.poseMisses };
}
