import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public'
import { error } from '@sveltejs/kit';
import * as api from '$lib/peertube-openapi';
import { loadClubDetails } from '$lib/stores/Clubs.svelte';

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, params }) => {
    if (typeof params.clubId !== "string") {
        error(404, 'no club selected');
    }
    // Get publicly available club data on the server for hydration.
    // The client needs to add private data, if available, at render rime.
    const club = await loadClubDetails(params.clubId, undefined, fetch);

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
        publicClubDetails: club,
        clubChannel,
    }
}