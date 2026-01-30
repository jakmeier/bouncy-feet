/**
 * Client sessions are available for registered users and guests.
 *
 * There can be multiple client sessions per user, when they use multiple
 * devices, browsers, or browser profiles.
 *
 * To create a new guest session, the client has to make a request to the API server.
 * This could be changed in the future, to allow offline initialization.
 */

import { browser } from "$app/environment";
import { ONBOARDING_STATE } from "$lib/onboarding";
import { requestNewGuestSession } from "$lib/stats";

export class ClientSession {
    /**
     * 
     * @param {ClientSessionData} clientSessionData 
     */
    constructor(clientSessionData) {
        this.clientSessionData = clientSessionData;
    }

    /**
     * Load from localStorage or create a new client session through the API.
     *
     * @param {import('$lib/sync').KvSync} kvSync
     * @return {Promise<ClientSession>}
     */
    static async initClientSession(kvSync) {
        const clientSessionData = await loadClientSessionAsync(kvSync);
        Object.assign(clientSession.clientSessionData, clientSessionData);

        // migration from old to new way of storing user meta in local storage
        await migrateFromFirstMetaStorage(kvSync);

        return clientSession;
    }

    /**
     * @returns {Object}
     */
    authHeader() {
        if (this.clientSessionData.id) {
            return {
                Authorization: `ClientSession ${this.clientSessionData.id}:${this.clientSessionData.secret}`,
            };
        }
        // This can happen when code calls this before clientSession is loaded.
        // Generally a programming error.
        console.warn(
            'Auth header could not be constructed,, missing client session'
        );
        return {};
    }

    /**
     * @param {string} key
     * @param {string} value
     * @param {string} type
     * @param {Date} _lastModified
     * @param {number} _version
     */
    async updateMetaInMemory(key, value, type, _lastModified, _version) {
        // for now, only handle strings here and avoid type conflicts
        // (the design might require some more iterations)
        if (type === 's:') {
            this.clientSessionData.meta[key] = value;
        }
    }
}

// This state is read-only. Updates must go through setters. Otherwise, they
// are not persisted.
// Actually initialized in initClientSession
/** @type {ClientSession} */
const clientSession = $state(new ClientSession({}));

/**
 * Load from localStorage or create a new client session through the API.
 * @return {Promise<ClientSessionData>}
 * @param {import('$lib/sync').KvSync} kvSync
 */
async function loadClientSessionAsync(kvSync) {
    if (!browser) {
        return {
            id: '',
            secret: '',
            meta: {},
        };
    }
    if (localStorage.clientSessionId) {
        return {
            id: localStorage.clientSessionId,
            secret: localStorage.clientSessionSecret,
            meta: kvSync.load(),
        };
    } else {
        return await requestNewGuestSession()
            .then((response) => {
                if (response.client_session_id && response.client_session_secret) {
                    /** @type {ClientSessionData} */
                    const newClientSession = {
                        id: response.client_session_id,
                        secret: response.client_session_secret,
                        meta: {
                            onboarding: ONBOARDING_STATE.FIRST_VISIT,
                        },
                    };
                    localStorage.clientSessionId = newClientSession.id;
                    localStorage.clientSessionSecret = newClientSession.secret;
                    kvSync.setStringValue(
                        'onboarding',
                        ONBOARDING_STATE.FIRST_VISIT,
                        new Date()
                    );
                    return newClientSession;
                } else {
                    console.error(
                        'Failed to create a guest session. Response:',
                        response
                    );
                    return {
                        id: '',
                        secret: '',
                        meta: {},
                    };
                }
            })
            .catch((err) => {
                console.error('Failed to create a guest session. Error:', err);
                loginError.title = 'profile.guest-login-failed';
                loginError.description = 'profile.guest-login-failed-description';
                return {
                    id: localStorage.clientSessionId,
                    secret: localStorage.clientSessionSecret,
                    meta: {
                        onboarding: ONBOARDING_STATE.FIRST_VISIT,
                    },
                };
            });
    }
}

/**
 * @param {import('$lib/sync').KvSync} kvSync
 */
async function migrateFromFirstMetaStorage(kvSync) {
    const oldMetaStr = localStorage.getItem('userMeta');
    if (oldMetaStr) {
        const oldMeta = parseOrNull(oldMetaStr);
        if (oldMeta) {
            // select an old date
            const date = new Date(2000, 0, 0, 0, 0);
            for (const [key, value] of Object.entries(oldMeta)) {
                // Only string values existed in the old version, so use that.
                // This also updates the remote.
                await kvSync.setStringValue(key, value, date);
            }
        }
    }
    localStorage.removeItem('userMeta');
}

/**
 * @param {string} key
 */
function parseOrNull(key) {
    try {
        return JSON.parse(key);
    } catch (e) {
        return null;
    }
}