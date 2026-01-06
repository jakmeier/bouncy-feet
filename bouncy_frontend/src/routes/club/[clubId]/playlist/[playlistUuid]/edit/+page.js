import { fetchPlaylist } from '$lib/peertube';
import { error } from '@sveltejs/kit';
import { client as peerTubeApi } from '$lib/peertube-openapi/client.gen';
import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, params }) => {
    if (!params.playlistUuid) {
        error(404, 'no playlist selected');
    }

    peerTubeApi.setConfig({
        baseUrl: PUBLIC_BF_PEERTUBE_URL,
    });

    /** @type {import('$lib/peertube-openapi').VideoPlaylist} */
    const playlist = await fetchPlaylist(params.playlistUuid);

    return {
        playlist
    }
}