import { loadChallengeSets } from '$lib/challenges.js';

// Challenge sets are WASM-backed objects, so load them in the browser.
export const ssr = false;

/** @type {import('./$types').PageLoad} */
export async function load({ parent }) {
  const { i18n } = await parent();
  return { challengeSets: await loadChallengeSets(i18n.locale) };
}
