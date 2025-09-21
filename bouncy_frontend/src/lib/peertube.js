/*
 * JS functions to access PeerTube API.
 *
 * API reference: https://docs.joinpeertube.org/api-rest-reference.html
 * 
 * Open API generator tool: https://github.com/hey-api/openapi-ts
 * (with this client): https://heyapi.dev/openapi-ts/clients/fetch
 */

import { PUBLIC_API_BASE, PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
import { client } from '$lib/peertube-openapi/client.gen';
import { getVideoPlaylistVideos } from './peertube-openapi';

const peerTubeUrl = PUBLIC_BF_PEERTUBE_URL;

client.setConfig({
    baseUrl: PUBLIC_BF_PEERTUBE_URL,
});

/**
 * @param {number} playlistId
 * @returns {Promise<import('./peertube-openapi').GetVideoPlaylistVideosResponse>}
 */
export async function fetchVideosOfPlaylist(playlistId, start = 0, count = 20) {
    /** @type {import('./peertube-openapi/client').Options<import('./peertube-openapi').GetVideoPlaylistVideosData>} */
    const options = {
        path: {
            playlistId
        },
        query: {
            start,
            count
        },
    };
    const { response, data } = await getVideoPlaylistVideos(options);

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Fetch videos failed: ${response.status} ${error}`);
    }

    if (!data) {
        const error = await response.text();
        throw new Error(`No data returned`);
    }

    return data;
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
