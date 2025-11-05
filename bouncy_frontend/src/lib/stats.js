import { PUBLIC_API_BASE } from '$env/static/public';

/** @param {string} path */
async function apiGetRequest(path) {
    const apiUrl = PUBLIC_API_BASE + path;

    try {
        const response = await fetch(apiUrl,
            {
                // Include cookies in the request to allow setting a session id
                // from the backend
                credentials: 'include',
            }
        );

        if (response.ok) {
            return await response.json();
        } else {
            console.error(`Failed GET request to ${apiUrl}`, response);
        }
    } catch (error) {
        console.error(`Error returned for GET request to ${apiUrl}`, error);
        return null;
    }
}

export async function requestNewGuestSession() {
    return await apiGetRequest('/new_guest_session');
}

/**
 * @param {string} endpoint
 * @param {object} options
 * @returns {Promise<ApiResponse>}
 */
export async function apiRequest(endpoint, options = {}) {
    let response;
    try {
        response = await fetch(`${PUBLIC_API_BASE}${endpoint}`, {
            ...options,
            credentials: 'include', // Include cookies in the request
        });
    }
    catch (err) {
        console.error('apiRequest fetch failed:', err);
        return {
            error: `Unknown fetch error ${err}`,
        };
    }

    if (response.ok) {
        return {
            okResponse: response
        };
    } else {
        const body = await response.text();

        switch (body) {
            case API_ERROR.UserNotFound: {
                // Some client sessions have been lost. They need to be replaced. Bubbling up.
                return {
                    error: body,
                    errorBody: body
                };
            }
            case API_ERROR.ClientSessionLoginNotAllow: {
                // Must use keycloak login. Bubbling up.
                return {
                    error: body,
                    errorBody: body
                };
            }
            case API_ERROR.ClientSessionOfDifferentUser: {
                // The locally stored data is for a different user. Bubbling up.
                return {
                    error: body,
                    errorBody: body
                };
            }
        }

        if (response.status === 401 || response.headers.get('WWW-Authenticate')) {
            // If unauthorized, need to redirect to the login endpoint on the api server.
            // Let the caller know to handle the redirect there.
            return {
                error: API_ERROR.NeedLogin,
            };
        }

        if (response.status === 502) {
            // Keycloak or Peertube or some other gateway is down.
            return {
                error: API_ERROR.BadGateway,
            };
        }

        return {
            error: `Unknown ${response.status} error`,
            errorBody: body
        };

    }
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

/**
 * @typedef {Object} ApiResponse
 * @property {API_ERROR} [error]
 * @property {string} [errorBody]
 * @property {Response} [okResponse]
 * 
 * @typedef {string} ApiError
 * @enum {ApiError}
 */
export const API_ERROR = {
    // directly from server
    NoAuthProvided: "NoAuthProvided",
    BadAuthHeader: "BadAuthHeader",
    ClientSessionLoginNotAllow: "ClientSessionLoginNotAllow",
    ClientSessionOfDifferentUser: "ClientSessionOfDifferentUser",
    ClientSessionHeaderMalformed: "ClientSessionHeaderMalformed",
    ClientSessionSecretMalformed: "ClientSessionSecretMalformed",
    UserNotFound: "UserNotFound",
    SubjectParsingFailed: "SubjectParsingFailed",
    DbError: "DbError",
    // not directly from server
    NeedLogin: "NeedLogin",
    BadGateway: "BadGateway",
    UnknownClientError: "UnknownClientError",
}