import { dev } from "$app/environment";
import { PUBLIC_API_BASE } from '$env/static/public';

let STATS_API_BASE = PUBLIC_API_BASE;

if (dev) {
    STATS_API_BASE = "http://localhost:3000";
}

const loginUrl = STATS_API_BASE + "/auth";

/**
 * @param {string} key
 * @param {string} value
 */
export async function submitUserMetadata(key, value) {
    const options = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            key_name: key,
            key_value: value,
            // chrono can parse the time including the timezone from this
            last_modified: new Date().toISOString(),
            // the only existing version for now
            version: 0,
        })
    };

    return await apiRequest('/user/meta/update', options);
}

/**
 * @param {UserData} user
 */
export async function submitStats(user) {
    if (!user.consentSendingStats) {
        return;
    }
    const apiUrl = STATS_API_BASE + '/user/stats';

    const payload = {
        id: user.id,
        name: user.publicName,
        steps: user.recordedSteps,
        seconds: Math.round(user.recordedSeconds),
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
        return [];
    }
}

/**
 * @param {string} endpoint
 * @param {object} options
 * @returns {Promise<Response>}
 */
export async function apiRequest(endpoint, options = {}) {
    const response = await fetch(`${STATS_API_BASE}${endpoint}`, {
        ...options,
        credentials: 'include', // Include cookies in the request
    });

    if (response.status === 401 || response.headers.get('WWW-Authenticate')) {
        // If unauthorized, redirect to the login endpoint on the api server
        window.location.href = loginUrl;
        return response;
    }

    if (!response.ok) {
        throw new Error(`API request failed with status ${response.status}`);
    }

    return response;
}

// This code is not UI or in any way browser API related, so it shouldn't really
// be in JS. At least I'm trying to follow such an architecture. But for now, it
// needs to be somewhere. Until I figure out how to manage data between users
// local storage and the server, this as good as any place to put it.

/// Accumulated experience per level for levels 0-9.
const LEVEL_ACC_EXP = [0, 10, 200, 450, 750, 1200, 1750, 2250, 3375, 4500];
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
        // Above 9, each level requires additional 15*i² experience, where i is
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
        // Above 9, each level requires additional 15*i² experience, where i is
        // the level.

        // WolframAlpha: 1/6(2n³ + 3n² + n - 1710) = accumulated sum of i^2 for i=10..n.
        // Hence, total_exp(lvl) = 4500 + 15/6 * (2n³ + 3n² + n - 1710)
        //        total_exp(lvl) =  225 +  2.5 * (2n³ + 3n² + n)
        const squared = level * level;
        const cubed = squared * level;
        return 225 + 2.5 * (2 * cubed + 3 * squared + level);
    }
}