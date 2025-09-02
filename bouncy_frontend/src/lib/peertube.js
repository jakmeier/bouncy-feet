import { PUBLIC_API_BASE, PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';

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
