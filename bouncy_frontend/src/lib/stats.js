// TODO: for production, point to hosted site
const STATS_API_BASE = "http://localhost:3000";

/**
 * @param {{ id: string; publicName: string; recordedSteps: number; recordedSeconds: number; recordedDances: number; }} user
 */
export async function submitStats(user) {
    const apiUrl = STATS_API_BASE + '/user/stats';

    const payload = {
        id: user.id,
        name: user.publicName,
        steps: user.recordedSteps,
        seconds: user.recordedSeconds,
        dances: user.recordedDances,
    };

    try {
        const response = await fetch(apiUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        });

        if (response.ok) {
        } else {
            console.error(`Post request to ${apiUrl} failed`);
        }
    } catch (error) {
        console.error(`Error during post request to ${apiUrl}:`, error);
    }
}

export async function fetchLeaderboard() {
    const apiUrl = STATS_API_BASE + '/scoreboard';

    try {
        const response = await fetch(apiUrl);

        if (response.ok) {
            return await response.json();
        } else {
            console.error('Failed to read scoreboard');
        }
    } catch (error) {
        console.error('Error while reading scoreboard:', error);
    }
}