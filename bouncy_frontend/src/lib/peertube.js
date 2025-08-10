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
    console.log('keycloak token is', token);

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
    const url = new URL(`${peerTubeUrl}/api/v1/users/token`);
    const externalAuthToken = body.externalAuthToken;
    const username = body.username;
    if (!externalAuthToken || !username) {
        console.error('response from token exchange is missing fields', body);
        return;
    }

    const params = new URLSearchParams();
    params.set('username', username);
    params.set('externalAuthToken', externalAuthToken);
    // TODO: read client id / secret dynamically
    params.set('client_id', 'yt4mum4a90rd5me7m0kzewv7eb5m2xk8');
    params.set('client_secret', 'KEJkeBDRKBg4wmE1kdLjftP2fGHJ0Quk');
    params.set('grant_type', 'password');
    const tokenResponse = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: params,
    });
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
}