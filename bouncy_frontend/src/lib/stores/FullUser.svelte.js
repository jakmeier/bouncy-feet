/**
 * A full user has a KeyCloak user with a verified email and a PeerTube account
 * that was created for them. They can upload videos, join clubs, and generally
 * access all part of the app without restrictions.
 *
 * A full user may or may not have an active access token for PeerTube access.
 */

import { PUBLIC_API_BASE, PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
import { fetchPeerTubeUser } from '$lib/peertube';
import { client as peerTubeApi } from '$lib/peertube-openapi/client.gen';
import { API_ERROR } from '$lib/stats';

// Pwa auth works independent of the api server, generating a token to be used
// for PeerTube only (for now).
/** @type {PwaAuth} */
const pwaAuth = $state({
    peerTubeToken: null,
});

const loggedInToApi = $derived(!!pwaAuth.peerTubeToken);
const isLoggedInToApi = () => loggedInToApi;

/** @type {import("$lib/peertube-openapi").User | {}} */
const peerTubeUser = $state({});

/**
 * @param {import('./ApiUser.svelte').ApiUser} apiUser
 * @returns {Promise<FullUser>}
 */
export async function initFullUser(apiUser) {
    return {
        isLoggedInToApi,
        logout,
        refreshPeerTubeUser,
        get peerTubeUser() {
            return (async () => {
                if (Object.keys(peerTubeUser).length === 0) {
                    await refreshPeerTubeUser();
                }
                return peerTubeUser;
            })();
        },
        pwaAuth,
        peerTubeToken: () => peerTubeToken(apiUser),
    };
}

/**
 * @param {import("./ApiUser.svelte").ApiUser} apiUser
 * 
 * @returns {Promise<BfError | { accessToken: string }>}
 */
async function peerTubeToken(apiUser) {

    if (!pwaAuth.peerTubeToken?.access_token) {
        const err = await refreshPeerTubeToken(apiUser);
        if (err) {
            return err;
        }
    }

    let accessToken = pwaAuth.peerTubeToken?.access_token;
    if (!accessToken) {
        console.error("refreshPeerTubeToken finished without error but still no access token available")
        return {
            title: 'profile.login-failed',
            description: '',
        };;
    }

    return { accessToken }
}

/**
 * @param {import("./ApiUser.svelte").ApiUser} apiUser
 * 
 * @returns {Promise<BfError | undefined>}
 */
async function refreshPeerTubeToken(apiUser) {
    const headers = {
        // Set a head even when it's an empty POST.
        // Triggers CORS pre-flight check to reduce CSRF risks.
        'Content-Type': 'application/json',
    };
    const body = '';

    peerTubeApi.setConfig({
        baseUrl: PUBLIC_BF_PEERTUBE_URL,
        auth: () => pwaAuth.peerTubeToken?.access_token,
    });

    try {
        const response = await apiUser.authenticatedApiRequest(
            'POST',
            '/peertube/token',
            headers,
            body
        );

        if (response.okResponse) {
            pwaAuth.peerTubeToken = await response.okResponse.json();
        } else if (response.error !== API_ERROR.NeedLogin) {
            console.error(response.error);
            const loginError = {
                title: 'profile.login-failed',
                description: '',
            };
            if (response.error === API_ERROR.BadGateway) {
                loginError.description = 'profile.login-failed-peertube-down';
            }
            return loginError;
        }
    } catch (err) {
        if (err.error === API_ERROR.BadGateway) {
            return {
                title: 'profile.login-failed',
                description: 'profile.login-failed-peertube-down',
            };;
        }
        console.error('unexpected error calling authenticatedApiRequest', err);
    }
}

async function refreshPeerTubeUser() {
    Object.assign(peerTubeUser, await fetchPeerTubeUser());
}

async function logout() {
    // TODO: handle local state without someone being logged in (without
    // creating a guest session -> be more explicit about guest sessions)
    pwaAuth.peerTubeToken = null;

    const currentUrl = window.location.href;
    window.location.assign(
        PUBLIC_API_BASE +
        '/logout?redirect_back_to=' +
        encodeURIComponent(currentUrl)
    );
}
