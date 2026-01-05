/*
 * JS functions to access PeerTube API.
 *
 * API reference: https://docs.joinpeertube.org/api-rest-reference.html
 * 
 * Open API generator tool: https://github.com/hey-api/openapi-ts
 * (with this client): https://heyapi.dev/openapi-ts/clients/fetch
 */

import * as api from '$lib/peertube-openapi';
import { getVideoPlaylistVideos, getUserInfo, uploadLegacy, uploadResumable, uploadResumableInit, uploadResumableCancel, getApiV1UsersMeVideos } from './peertube-openapi';

/**
 * @param {string} playlistUuid
 * @returns {Promise<api.GetApiV1VideoPlaylistsByPlaylistIdResponse>}
 */
export async function fetchPlaylist(playlistUuid) {
    /** @type {api.Options<api.GetApiV1VideoPlaylistsByPlaylistIdData>} */
    const options = {
        path: {
            // @ts-ignore: string also works here, even if not included in the
            // openapi generated code. Using the numeric id DOES NOT work for
            // unlisted playlists.
            playlistId: playlistUuid
        },
    };
    const { response, data } = await api.getApiV1VideoPlaylistsByPlaylistId(options);

    if (!response.ok) {
        // Body has already been consumed...
        // const error = await response.text();
        throw new Error(`Fetch playlist failed: ${response.status}`);
    }

    if (!data) {
        const error = await response.text();
        throw new Error(`No data returned ${error}`);
    }

    return data;
}

/**
 * @param {string} playlistUuid
 * @returns {Promise<api.GetVideoPlaylistVideosResponse>}
 */
export async function fetchVideosOfPlaylist(playlistUuid, start = 0, count = 20) {
    /** @type {api.Options<api.GetVideoPlaylistVideosData>} */
    const options = {
        path: {
            // @ts-ignore: string also works here, even if not included in the
            // openapi generated code. Using the numeric id DOES NOT work for
            // unlisted playlists.
            playlistId: playlistUuid
        },
        query: {
            start,
            count
        },
    };
    const { response, data } = await getVideoPlaylistVideos(options);

    if (!response.ok) {
        //  Body has already been consumed...
        // const error = await response.text();
        throw new Error(`Fetch videos failed: ${response.status}`);
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
 * @readonly
 * @enum {api.VideoPrivacySet} 
 */
export const VIDEO_PRIVACY = {
    /** @type {api.VideoPrivacySet} */
    PUBLIC: 1,
    /** @type {api.VideoPrivacySet} */
    UNLISTED: 2,
    /** @type {api.VideoPrivacySet} */
    PRIVATE: 3,
}

/**
 * @param {File} file
 * @param {number} channelId
 * @param {(ratio: number) => void} onProgress
 * @param {api.VideoPrivacySet} [privacy]
 * @returns {Promise<api.VideoUploadResponse>}
*/
export async function uploadVideoToPeerTubeResumable(file, channelId, onProgress, privacy = VIDEO_PRIVACY.PRIVATE) {
    const totalSize = file.size;
    let chunkSize = initialChunkSize(totalSize);

    const { response, error } = await uploadResumableInit({
        body: {
            // video name
            name: file.name,
            // file name with extension
            filename: file.name,
            channelId,
            privacy,
            description: 'Uploaded from Bouncy Feet',
        },
        headers: {
            'X-Upload-Content-Length': totalSize,
            'X-Upload-Content-Type': file.type,
        }
    });


    const uploadId = response.headers.get('Location')?.split('upload_id=')[1];
    if (!uploadId) throw new Error(`No upload_id returned, ${JSON.stringify(error)}`);
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

        const chunkStartT = performance.now();
        const { response, data, error } = await uploadResumable({
            body: chunk,
            query: { upload_id: uploadId },
            headers: {
                'Content-Range': contentRange,
                'Content-Length': chunk.size,
            },
        });
        const chunkTimeMs = performance.now() - chunkStartT;

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
            const retryAfter = response.headers.get('Retry-After');
            if (retryAfter) {
                // We hit the rate-limit.
                // https://docs.joinpeertube.org/api-rest-reference.html#section/Rate-limits
                // "Retry-After" header should always be available, while
                // "X-RateLimit-Remaining" and "X-RateLimit-Reset" are not
                // available to scrips by default, as they are not listed as
                // "Access-Control-Expose-Headers" by PeerTube.
                const waitSeconds = Number.parseInt(retryAfter) || 10;
                console.debug(`Rate-limited, delaying upload for ${waitSeconds}s`);
                await new Promise((r) => setTimeout(r, waitSeconds * 1000));
                continue;
            }
            throw new Error(`Video uploading failed. ${error}`);
        }
        // Range header should return the progress, e.g. bytes=0-1024 means the server already has the first 1024 bytes.
        const serverProgress = response.headers.get('Range')?.split('-')[1];
        offset = Number.parseInt(serverProgress || end.toString());
        onProgress(offset / totalSize);

        // don't spam the server too much, use large chunks on god internet
        chunkSize = updatedChunkSize(chunkTimeMs, chunkSize);
    }
}

/**
 * @param {number} totalSize
 * @returns {number} chunkSize
 */
function initialChunkSize(totalSize) {
    const REQUEST_LIMIT = 40; // Rate-limit is at 50 requests / 10 seconds. Keep it well under.
    const MIN_CHUNK_SIZE = 1024 * 1024; // 1 MB seems like a good minimum.
    const MAX_CHUNK_SIZE = 1024 * 1024 * 512; // 512 MB

    // Keep it under the limit but make it a power of 2;
    const exponent = Math.ceil(Math.log2(totalSize / REQUEST_LIMIT));
    const chunkSize = Math.pow(2, exponent);

    if (chunkSize > MAX_CHUNK_SIZE) {
        throw new Error("Upload too big");
    }

    return Math.max(chunkSize, MIN_CHUNK_SIZE);
}

/**
 * @param {number} chunkTimeMs
 * @param {number} chunkSize
 * @returns {number} chunkSize
 */
function updatedChunkSize(chunkTimeMs, chunkSize) {
    const tooFast = 200;
    const maxChunkSize = 1024 * 1024 * 128;
    if (chunkTimeMs < tooFast && chunkSize * 2 <= maxChunkSize) {
        return chunkSize * 2;
    } else {
        return chunkSize;
    }
}

/**
 * @param {number} start -- pagination offset
 * @param {number} count -- pagination size
 * @returns {Promise<Array<api.Video>>}
 */
export async function fetchMyVideos(start = 0, count = 20) {
    const options = {
        query: {
            start,
            count
        }
    };
    const { response, data, error } = await getApiV1UsersMeVideos(options);

    if (!response.ok) {
        const errText = await response.text();
        throw new Error(`Upload failed: ${response.status} ${errText} ${error}`);
    }

    if (!data || !data.data) {
        const error = await response.text();
        throw new Error(`No data returned ${error}`);
    }

    return data.data;
}

/**
 * @typedef {object} UpdateVideoArgs
 * @prop {number} [channelId] -- only works for channels of the same account
 * @prop {Blob | File} [thumbnailfile]
 * @prop {Blob | File} [previewfile]
 * @prop {api.VideoCategorySet} [category]
 * @prop {api.VideoLicenceSet} [licence]
 * @prop {api.VideoLanguageSet} [language]
 * @prop {api.VideoPrivacySet} [privacy]
 * @prop {string} [description]
 * @prop {string} [waitTranscoding]
 * @prop {string} [support]
 * @prop {boolean} [nsfw]
 * @prop {unknown} [nsfwSummary]
 * @prop {api.NsfwFlag} [nsfwFlags]
 * @prop {string} [name]
 * @prop {Array<string>} [tags]
 * @prop {api.VideoCommentsPolicySet} [commentsPolicy]
 * @prop {boolean} [downloadEnabled]
 * @prop {string | null} [originallyPublishedAt]
 * @prop {api.VideoScheduledUpdate} [scheduleUpdate]
 * @prop {api.AddVideoPasswords} [videoPasswords]
 */

/**
 * @param {number | string} id -- id (integer) or UUIDv4 (string) or shortUUID (string)
 * @param {UpdateVideoArgs} args -- fields to update
 * @returns {Promise<boolean>} -- true if successful
 */
export async function updateVideo(id, args) {
    try {
        const options = {
            path: {
                id
            },
            body: {
                ...args
            }
        };
        await api.putVideo(options);
        return true;
    } catch (err) {
        console.error("Failed updating video", err);
    }
    return false;
}