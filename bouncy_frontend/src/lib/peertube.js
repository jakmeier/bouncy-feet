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
import { getVideoPlaylistVideos, getUserInfo, uploadLegacy, uploadResumable, uploadResumableInit, uploadResumableCancel } from './peertube-openapi';

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

/** 
 *  @param {File} file
 *  @param {number} channelId
 *  @param {(ratio: number) => void} onProgress
 *  @returns {Promise<api.VideoUploadResponse>}
*/
export async function uploadVideoToPeerTubeResumable(file, channelId, onProgress) {
    // 1 MB seems fine but could be optimized and made dynamic
    const chunkSize = 1024 * 1024;
    const totalSize = file.size;

    const { response, error } = await uploadResumableInit({
        body: {
            // video name
            name: file.name,
            // file name with extension
            filename: file.name,
            channelId,
            // 1 = Public, 2 = Unlisted, 3 = Private
            privacy: 3,
            description: 'Uploaded from Bouncy Feet',
        },
        headers: {
            'X-Upload-Content-Length': totalSize,
            'X-Upload-Content-Type': file.type,
        }
    });


    const uploadId = response.headers.get('Location')?.split('upload_id=')[1];
    if (!uploadId) throw new Error(`No upload_id returned, ${error}`);
    let offset = Number.parseInt(response.headers.get('Content-Length') || '0');

    // expecting 201 responses
    if (response.status === 200) {
        //already exists need to resume
        const { response, error } = await uploadResumable({
            query: { upload_id: uploadId },
            headers: {
                'Content-Range': `bytes */${totalSize}`,
                'Content-Length': 0,
            },
        });
        if (response.status !== 308) {
            throw new Error(`Video uploading failed on resume attempt. ${error}`);
        }
        // Range header should return the progress, e.g. bytes=0-1024 means the server already has the first 1024 bytes.
        const serverProgress = response.headers.get('Range')?.split('-')[1];
        offset = Number.parseInt(serverProgress || '0');
    } else if (response.status !== 201) {
        throw new Error(`Video uploading failed with unexpected status ${response.status} ${response.text}. ${error}`);
    }


    while (true) {
        const end = Math.min(offset + chunkSize, totalSize);
        const chunk = file.slice(offset, end);
        const contentRange = `bytes ${offset}-${end - 1}/${totalSize}`;

        const { response, data, error } = await uploadResumable({
            body: chunk,
            query: { upload_id: uploadId },
            headers: {
                'Content-Range': contentRange,
                'Content-Length': chunk.size,
            },
        });

        if (response.status === 200) {
            if (!data) {
                throw new Error(`Video uploaded but no response. ${error}`);
            }
            return data;
        }

        // PeerTube uses this non-standard, google drive like, resumable protocol
        // https://github.com/kukhariev/node-uploadx/blob/master/proto.md
        // 308 in this context means 'Resume Incomplete' and not the standard '308 Permanent Redirect'
        if (response.status !== 308) {
            throw new Error(`Video uploading failed. ${error}`);
        }
        // Range header should return the progress, e.g. bytes=0-1024 means the server already has the first 1024 bytes.
        const serverProgress = response.headers.get('Range')?.split('-')[1];
        offset = Number.parseInt(serverProgress || end.toString());
        onProgress(offset / totalSize);
    }
}