import { PUBLIC_API_BASE } from '$env/static/public'
import { error } from '@sveltejs/kit';

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
     * @prop {PlaylistInfo} main_playlist
     * @prop {PlaylistInfo[]} public_playlists
     * @prop {PlaylistInfo[]} private_playlists
     */
    /** 
    * @type {ClubDetailsResponse}
    */
    const data = await response.json();

    return {
        admins: data.admins,
        members: data.members,
        main_playlist: data.main_playlist,
        public_playlists: data.public_playlists,
        private_playlists: data.private_playlists,
    }
}