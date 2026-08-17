/**
 * Map a wall-clock delay to Web Audio time while compensating output latency.
 *
 * @param {number} currentTime Current AudioContext time in seconds.
 * @param {number} delaySeconds Delay from performance.now() to the target.
 * @param {number} outputLatency Audio output latency in seconds.
 */
export function resolveAudioSchedule(currentTime, delaySeconds, outputLatency) {
  const requestedStart = currentTime + delaySeconds - outputLatency;
  return {
    start: Math.max(currentTime, requestedStart),
    clampedSeconds: Math.max(0, currentTime - requestedStart),
  };
}
