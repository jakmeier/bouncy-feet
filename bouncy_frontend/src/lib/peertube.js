/*
 * JS functions to access PeerTube API.
 *
 * API reference: https://docs.joinpeertube.org/api-rest-reference.html
 * 
 * Open API generator tool: https://github.com/hey-api/openapi-ts
 * (with this client): https://heyapi.dev/openapi-ts/clients/fetch
 */

import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
import * as api from '$lib/peertube-openapi';
import { getVideoPlaylistVideos, getUserInfo, uploadLegacy } from './peertube-openapi';

const peerTubeUrl = PUBLIC_BF_PEERTUBE_URL;

/**
 * @param {number} playlistId
 * @returns {Promise<api.GetVideoPlaylistVideosResponse>}
 */
export async function fetchVideosOfPlaylist(playlistId, start = 0, count = 20) {
    /** @type {api.Options<api.GetVideoPlaylistVideosData>} */
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
        throw new Error(`No data returned ${error}`);
    }

    return data;
}

/**
 * @returns {Promise<api.User>}
 */
export async function fetchPeerTubeUser() {
    const { response, data } = await getUserInfo();

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`getUserInfo failed: ${response.status} ${error}`);
    }

    if (!data) {
        const error = await response.text();
        throw new Error(`No data returned ${error}`);
    }

    // @ts-ignore generated types are wrong, it doesn't return an array
    return data;
}

/** 
 *  @param {File} file
 *  @param {number} channelId
 *  @returns {Promise<api.VideoUploadResponse>}
*/
export async function uploadVideoToPeerTube(
    file,
    channelId,
) {

    /** @type {api.Options<api.UploadLegacyData>} */
    const options = {
        body: {
            videofile: file,
            name: file.name,
            channelId,
            // 1 = Public, 2 = Unlisted, 3 = Private
            privacy: 3,
            description: 'Uploaded from Bouncy Feet',
        }
    };

    // TODO: using the resumable API would be better
    const { response, data } = await uploadLegacy(options);

    if (!response.ok) {
        const error = await response.text();
        throw new Error(`Upload failed: ${response.status} ${error}`);
    }

    if (!data) {
        const error = await response.text();
        throw new Error(`No data returned ${error}`);
    }

    return data;
}
