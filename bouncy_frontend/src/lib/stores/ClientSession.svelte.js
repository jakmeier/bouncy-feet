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
import { apiRequest, requestNewClientSession, requestNewGuestSession } from "$lib/stats";

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
     * @param {boolean} createGuest
     * @return {Promise<ClientSession|null>}
     */
    static async initClientSession(createGuest) {
        let clientSessionData = await loadClientSessionAsync();

        if (!clientSession && createGuest) {
            const newSession = await requestNewGuestSession();
            clientSessionData = await storeNewSession(newSession);
        }

        if (!clientSessionData) {
            return null;
        }

        Object.assign(clientSession.clientSessionData, clientSessionData);

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
     * @param {import('$lib/sync').KvSync} kvSync
     */
    async migrateFromFirstMetaStorage(kvSync) {
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
}

// This state is read-only. Updates must go through setters. Otherwise, they
// are not persisted.
// Actually initialized in initClientSession
/** @type {ClientSession} */
const clientSession = $state(new ClientSession({}));

/**
 * Load from localStorage or create a new client session through the API for an existing user.
 * @return {Promise<ClientSessionData|null>}
 */
async function loadClientSessionAsync() {
    if (!browser) {
        return {
            id: '',
            secret: '',
        };
    }
    if (localStorage.clientSessionId) {
        return {
            id: localStorage.clientSessionId,
            secret: localStorage.clientSessionSecret,
        };
    } else {
        // nothing in local storage
        // but we might already be logged in to the API server and just need to create a new session
        const infoResponse = await apiRequest('/user');
        if (infoResponse.okResponse) {
            const userInfo = await infoResponse.okResponse.json();
            if (!userInfo || userInfo.sub === undefined) {
                throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
            }

            // If we can read our private information, then we can also create a
            // new session for the user.
            const newSession = await requestNewClientSession();
            return await storeNewSession(newSession);
        }
        return null;
    }
}

/**
 * Store client session in local storage after it was created over the API.
 *
 * Could be a guest session or one for a full user.
 *
 * @param {{client_session_id: string, client_session_secret: string }} response
 * @return {Promise<ClientSessionData>}
 */
async function storeNewSession(response) {


    /** @type {ClientSessionData} */
    const newClientSession = {
        id: response.client_session_id,
        secret: response.client_session_secret
    };
    localStorage.clientSessionId = newClientSession.id;
    localStorage.clientSessionSecret = newClientSession.secret;
    return newClientSession;
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