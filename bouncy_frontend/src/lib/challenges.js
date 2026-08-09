import { parseChallengeString } from '$lib/instructor/bouncy_instructor';

// Discover bundled challenge sets so adding one only requires adding its RON file.
const challengeFiles = /** @type {Record<string, () => Promise<string>>} */ (
  import.meta.glob('$lib/assets/challenges/*.ron', {
    query: '?raw',
    import: 'default',
  })
);

/**
 * @param {string} locale
 * @returns {Promise<import('$lib/instructor/bouncy_instructor').ChallengeSet[]>}
 */
export async function loadChallengeSets(locale) {
  const sets = await Promise.all(
    Object.values(challengeFiles).map(async (loadFile) => {
      try {
        return parseChallengeString(await loadFile(), locale);
      } catch (error) {
        console.error('Could not load challenge set', error);
        return undefined;
      }
    })
  );

  return /** @type {import('$lib/instructor/bouncy_instructor').ChallengeSet[]} */ (
    sets.filter(Boolean)
  );
}
