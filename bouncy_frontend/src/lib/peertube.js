import { getToken } from '$lib/keycloak';
import { pwaAuth } from '$lib/stores/Auth.svelte';
import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';

const peerTubeUrl = PUBLIC_BF_PEERTUBE_URL;
// const peerTubeUrl = 'http://localhost:9000';

/**
 * @typedef {Object} PlaylistVideos
 * 
 * @property {number} total
 * @property {PlaylistVideo[]} data
 *
 * @typedef {Object} PlaylistVideo 
 * @property {number} id
 * @property {number} position
 * @property {Video} video
 *
 * @typedef {Object} Video
 * @property {number} id
 * @property {string} uuid
 * 
 * More properties:
 * https://docs.joinpeertube.org/api-rest-reference.html#tag/Video-Playlists/operation/getVideoPlaylistVideos
 */


// Default should work for prod but will be dynamically updated if it fails.
const peerTubeAuthClientConfig = {
    clientId: 'uy47nbv6ikmg0m7c8y1rr6gvtq3dytqp',
    // Not actually secret, this can be queried from a public endpoint.
    // This has no security purpose, just getting through the given API for external auth plugins.
    clientSecret: 'W0ezWGF4ZS6rJW0hRM91Atht3W2E7BdD'
};

/**
 * @param {string} playlistId
 * 
 * @returns {Promise<PlaylistVideos>}
 */
export async function fetchVideosOfPlaylist(playlistId, start = 0, count = 20) {

    const res = await fetch(`${peerTubeUrl}/api/v1/video-playlists/${playlistId}/videos`);

    if (!res.ok) {
        const error = await res.text();
        throw new Error(`Fetch videos failed: ${res.status} ${error}`);
    }

    /** @type {PlaylistVideos} */
    const result = await res.json();

    return result;
}

/** 
 *  @param {File} file
 *  @param {string} accessToken
*/
export async function uploadVideoToPeerTube(
    file,
    accessToken,
) {
    const uploadFormData = new FormData();
    // TODO: find channel id for user
    uploadFormData.append('channelId', "3");
    uploadFormData.append('videofile', file);
    uploadFormData.append('name', file.name);
    uploadFormData.append('privacy', '3'); // 1 = Public, 2 = Unlisted, 3 = Private
    uploadFormData.append('description', 'Uploaded from Bouncy Feet!');

    const res = await fetch(`${peerTubeUrl}/api/v1/videos/upload`, {
        method: 'POST',
        headers: {
            Authorization: `Bearer ${accessToken}`
        },
        body: uploadFormData
    });

    if (!res.ok) {
        const error = await res.text();
        throw new Error(`Upload failed: ${res.status} ${error}`);
    }

    const result = await res.json();
    return result;
}

export async function loginToPeertube() {
    const token = getToken();
    if (!token) {
        console.warn('got no keycloak token');
        return;
    }

    const options = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
            grant_type: 'urn:ietf:params:oauth:grant-type:jwt-bearer',
            assertion: token?.toString(),
        }),
        redirect: 'manual',
    };
    const response = await fetch(
        `${peerTubeUrl}/plugins/auth-openid-connect/router/token-exchange`,
        options
    );

    const peerTubeToken = await bypassTokenToBearer(response);
    if (peerTubeToken) {
        pwaAuth.peerTubeToken = peerTubeToken;
    }
}

/**
 * @param {Response} tokenExchangeResponse
 */
async function bypassTokenToBearer(tokenExchangeResponse) {
    // What follows is the custom token exchange required on PeerTube with the openid plugin.
    if (tokenExchangeResponse.status !== 200) {
        console.error(
            'unexpected response from token exchange',
            tokenExchangeResponse.status,
            tokenExchangeResponse.statusText,
            tokenExchangeResponse
        );
        return;
    }

    const body = await tokenExchangeResponse.json();
    const externalAuthToken = body.externalAuthToken;
    const username = body.username;
    if (!externalAuthToken || !username) {
        console.error('response from token exchange is missing fields', body);
        return;
    }

    let tokenResponse = await sendTokenRequest(username, externalAuthToken);

    // error details are reported with content type "application/problem+json; charset=utf-8"
    // checking just for "json" for robustness to changes in how the content header is set by the server
    if (tokenResponse.status === 400 && tokenResponse.headers.get("Content-Type")?.includes("json")) {
        const error = await tokenResponse.json();
        if (error.code && error.code === "invalid_client") {
            console.info("got an invalid_client response, fetching the latest client config and trying again");
            await updateOauthClientConfig();
            tokenResponse = await sendTokenRequest(username, externalAuthToken);
        }
    }

    if (tokenResponse.status !== 200) {
        console.error(
            'request getting PeerTube OAuth token failed',
            tokenResponse.status,
            tokenResponse
        );
        return;
    }
    const token = await tokenResponse.json();
    if (!token) {
        console.error(
            'PeerTube OAuth token response has no token',
            tokenResponse.status,
            tokenResponse
        );
        return;
    }
    return token;

    /**
     * @param {string} username
     * @param {string} externalAuthToken
     */
    async function sendTokenRequest(username, externalAuthToken) {
        const url = new URL(`${peerTubeUrl}/api/v1/users/token`);
        const params = new URLSearchParams();
        params.set('username', username);
        params.set('externalAuthToken', externalAuthToken);
        params.set('client_id', peerTubeAuthClientConfig.clientId);
        params.set('client_secret', peerTubeAuthClientConfig.clientSecret);
        params.set('grant_type', 'password');
        const tokenResponse = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: params,
        });
        return tokenResponse;
    }
}

async function updateOauthClientConfig() {
    const response = await fetch(`${peerTubeUrl}/api/v1/oauth-clients/local`);
    if (response.status !== 200) {
        console.error('request getting PeerTube OAuth client config failed', response.status, response);
        return false;
    }

    const config = await response.json();
    if (!config.client_id || !config.client_secret) {
        console.error('PeerTube OAuth client config misses fields', config);
        return false;
    }

    peerTubeAuthClientConfig.clientId = config.client_id;
    peerTubeAuthClientConfig.clientSecret = config.client_secret;

    return true;
}