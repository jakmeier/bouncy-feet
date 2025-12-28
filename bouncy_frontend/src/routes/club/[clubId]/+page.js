import { PUBLIC_API_BASE, PUBLIC_BF_PEERTUBE_URL } from '$env/static/public'
import { error } from '@sveltejs/kit';
import * as api from '$lib/peertube-openapi';

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, params }) => {
    if (typeof params.clubId !== "string") {
        error(404, 'no club selected');
    }
    const response = await fetch(`${PUBLIC_API_BASE}/clubs/${params.clubId}`);
    if (!response.ok) {
        console.error("failed fetching data", response);
        error(502, 'bad gateway');
    }

    /**
     * @typedef {object} ClubDetailsResponse
     * @prop {PublicUserResponse[]} admins
     * @prop {PublicUserResponse[]} members
     * @prop {number} [channel_id]
     * @prop {string} [channel_handle]
     * @prop {PlaylistInfo} [main_playlist]
     * @prop {PlaylistInfo[]} public_playlists
     * @prop {PlaylistInfo[]} private_playlists
     * @prop {string} [web_link]
     */
    /** 
    * @type {ClubDetailsResponse}
    */
    const club = await response.json();

    /** @type {api.VideoChannel | null} */
    let clubChannel = null;
    // could use getVideoChannel() but that wouldn't use svelte's `fetch`
    if (club.channel_handle) {
        const channelResponse = await fetch(`${PUBLIC_BF_PEERTUBE_URL}/api/v1/video-channels/${club.channel_handle}`);
        if (!channelResponse.ok) {
            console.error('failed fetching channel data', channelResponse);
            error(502, 'bad gateway');
        }

        /** @type {api.VideoChannel} */
        clubChannel = await channelResponse.json();
    }

    return {
        clubDetails: club,
        clubChannel,
    }
}