import { dev } from "$app/environment";

// TODO: This is a temporary place to host the stats service
let STATS_API_BASE = "https://demo.paddlers.ch:3000";

if (dev) {
    STATS_API_BASE = "http://localhost:3000";
}

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

// This code is not UI or in any way browser API related, so it shouldn't really
// be in JS. At least I'm trying to follow such an architecture. But for now, it
// needs to be somewhere. Until I figure out how to manage data between users
// local storage and the server, this as good as any place to put it.

/// Accumulated experience per level for levels 0-9.
const LEVEL_ACC_EXP = [0, 10, 300, 1500, 3000, 9000, 13500, 20250, 30375, 45000]
/**
 * Convert from a total experience amount to a level.
 * 
 * @param {number} exp
 * @returns {number} level
 */
export function experienceToLevel(exp) {
    if (exp < LEVEL_ACC_EXP[LEVEL_ACC_EXP.length - 1]) {
        return LEVEL_ACC_EXP.findIndex((x) => exp < x) - 1;
    }

    let level = 9;
    let toNext = incrementalExperienceForLevel(level + 1);
    while (toNext <= exp) {
        exp -= toNext;
        level += 1;
        toNext = incrementalExperienceForLevel(level + 1);
    }
    return level;
}

/**
 * How much experience it takes to reach a certain level from the previous
 * level.
 *
 * @param {number} level
 * @returns {number} exp
 */
export function incrementalExperienceForLevel(level) {
    if (level == 0) {
        return 0;
    }
    else if (level < LEVEL_ACC_EXP.length) {
        return LEVEL_ACC_EXP[level] - LEVEL_ACC_EXP[level - 1];
    } else {
        // Above 9, each level requires additional 150*i² experience, where i is
        // the level.
        return level * level * 150;
    }
}

/**
 * How much total experience it takes to reach a certain level.
 *
 * @param {number} level
 * @returns {number} exp
 */
export function totalExperienceForLevel(level) {
    if (level == 0) {
        return 0;
    }
    else if (level < LEVEL_ACC_EXP.length) {
        return LEVEL_ACC_EXP[level] - LEVEL_ACC_EXP[level - 1];
    } else {
        // Above 9, each level requires additional 150*i² experience, where i is
        // the level.

        // WolframAlpha: 1/6(2n³ + 3n² + n - 1710) = accumulated sum of i^2 for i=10..n.
        // Hence, total_exp(lvl) = 45000 + 150/6 * (2n³ + 3n² + n - 1710)
        //        total_exp(lvl) =  2250 +    25 * (2n³ + 3n² + n)
        const squared = level * level;
        const cubed = squared * level;
        return 2250 + 25 * (2 * cubed + 3 * squared + level);
    }
}