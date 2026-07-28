import { describe, expect, it, vi } from 'vitest';
import {
  challengeProgressKey,
  completeChallenge,
  deriveChallengeProgress,
  readChallengeProgress,
} from './challengeProgress.js';

describe('challenge progress', () => {
  it('updates reactive user metadata before the save resolves, unlocking dependents immediately', async () => {
    /** @type {((result: {updated: boolean, synced: boolean}) => void) | undefined} */
    let resolveSave;
    const apiUser = {
      /** @type {Record<string, string>} */
      meta: {},
      setUserMeta: vi.fn(
        () =>
          new Promise((resolve) => {
            resolveSave = resolve;
          })
      ),
    };
    const set = { id: 'test-set', definitionVersion: 1 };
    const challenges = [
      { id: 'first', prerequisites: [] },
      { id: 'second', prerequisites: ['first'] },
    ];

    const completion = completeChallenge(apiUser, set, 'first');
    const raw = apiUser.meta[challengeProgressKey(set.id)];
    const progress = readChallengeProgress(raw, set.definitionVersion);

    expect(progress.completed.first).toBeTruthy();
    expect(
      deriveChallengeProgress(challenges, progress.completed).states.second
    ).toMatchObject({
      available: true,
      locked: false,
    });

    resolveSave?.({ updated: true, synced: true });
    await expect(completion).resolves.toMatchObject({ syncPending: false });
    expect(apiUser.setUserMeta).toHaveBeenCalledWith(
      challengeProgressKey(set.id),
      raw
    );
  });

  it('reports a pending remote sync without locking the next level again', async () => {
    const apiUser = {
      /** @type {Record<string, string>} */
      meta: {},
      setUserMeta: vi.fn(async () => ({ updated: true, synced: false })),
    };
    const set = { id: 'test-set', definitionVersion: 1 };

    await expect(
      completeChallenge(apiUser, set, 'first')
    ).resolves.toMatchObject({
      syncPending: true,
    });
    expect(apiUser.meta[challengeProgressKey(set.id)]).toBeTruthy();
  });

  it('does not leave an unlocked local path when persistence rejects the update', async () => {
    const apiUser = {
      /** @type {Record<string, string>} */
      meta: {},
      setUserMeta: vi.fn(async () => ({ updated: false, synced: true })),
    };
    const set = { id: 'test-set', definitionVersion: 1 };

    await expect(completeChallenge(apiUser, set, 'first')).rejects.toThrow(
      'superseded'
    );
    expect(apiUser.meta[challengeProgressKey(set.id)]).toBeUndefined();
  });
});
