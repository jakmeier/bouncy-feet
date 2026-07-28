// @ts-nocheck
/** Derived challenge progression. Only completion facts belong in storage. */
export function challengeProgressKey(setId) { return `challenge-progress:${setId}`; }

export function readChallengeProgress(raw, definitionVersion) {
  try {
    const value = raw ? JSON.parse(raw) : {};
    return { schemaVersion: 1, definitionVersion, completed: value.completed && typeof value.completed === 'object' ? value.completed : {} };
  } catch (_) {
    return { schemaVersion: 1, definitionVersion, completed: {} };
  }
}

export function deriveChallengeProgress(challenges, completed) {
  const states = Object.fromEntries(challenges.map(challenge => {
    const done = Boolean(completed[challenge.id]);
    const missing = challenge.prerequisites.filter(id => !completed[id]);
    return [challenge.id, { completed: done, locked: !done && missing.length > 0, available: !done && missing.length === 0, missing }];
  }));
  return { states, won: challenges.length > 0 && challenges.every(challenge => states[challenge.id].completed) };
}

export async function completeChallenge(apiUser, set, challengeId) {
  const key = challengeProgressKey(set.id);
  const progress = readChallengeProgress(apiUser.meta[key], set.definitionVersion);
  if (!progress.completed[challengeId]) {
    progress.completed[challengeId] = new Date().toISOString();
    const value = JSON.stringify(progress);

    // The overview derives its cards from `apiUser.meta`. Update that
    // canonical reactive value before waiting for the remote sync, so the
    // next level unlocks immediately when the user returns to the path.
    const previousValue = apiUser.meta[key];
    apiUser.meta[key] = value;
    try {
      const stored = await apiUser.setUserMeta(key, value);
      if (!stored.updated) {
        throw new Error('Challenge completion was superseded by newer progress');
      }
      return { progress, syncPending: !stored.synced };
    } catch (error) {
      apiUser.meta[key] = previousValue;
      throw error;
    }
  }
  return { progress, syncPending: false };
}
