import { fetchLeaderboard } from '$lib/stats';

// fetchLeaderboard doesn't work on SSR.
export const ssr = false;
// Profile of a user should not be prerendered
export const prerender = false;

export async function load() {
    return {
        leaderboard: await fetchLeaderboard()
    };
}