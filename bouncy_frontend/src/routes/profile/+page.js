import { fetchLeaderboard } from '$lib/stats';

export async function load() {
    return {
        leaderboard: await fetchLeaderboard()
    };
}