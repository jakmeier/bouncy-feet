// @ts-nocheck
// Challenge sets are WASM-backed objects and cannot be serialized through SSR.
// Load them in the browser so their methods (including `tracker`) remain live.
export const ssr = false;

import { parseChallengeString } from '$lib/instructor/bouncy_instructor';

/** Load every bundled challenge set. Content is intentionally discovered rather
 * than hard-coded so adding a set requires only adding its RON file. */
const challengeFiles = import.meta.glob('$lib/assets/challenges/*.ron', { query: '?raw', import: 'default' });

export async function load({ parent }) {
  const { i18n } = await parent();
  const sets = (await Promise.all(Object.values(challengeFiles).map(async loadFile => {
    try { return parseChallengeString(await loadFile(), i18n.locale); }
    catch (error) { console.error('Could not load challenge set', error); return undefined; }
  }))).filter(Boolean);
  return { challengeSets: sets };
}
