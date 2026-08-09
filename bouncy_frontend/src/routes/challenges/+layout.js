// Challenge sets are WASM-backed objects and cannot be serialized through SSR.
// Load them in the browser so their methods (including `tracker`) remain live.
export const ssr = false;

import { loadChallengeSets } from '$lib/challenges.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ parent }) {
  const { i18n } = await parent();
  return { challengeSets: await loadChallengeSets(i18n.locale) };
}
