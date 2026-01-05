import { PUBLIC_API_BASE } from "$env/static/public";
import { apiRequest } from "$lib/stats";
import { error } from "@sveltejs/kit";
import { getContext } from "svelte";

/** @returns {ClubsContextData} */
export function getClubsContext() {
    const ctx = getContext('clubs');
    if (!ctx) {
        console.warn("missing clubs context");
    }
    return ctx;
}

/** @type {ClubsData} */
export const clubsData = $state({
    mine: [],
    public: [],
    currentClubDetails: undefined,
    loadedForUser: false,
    // TODO: consider adding
    // lastUpdated:
})

/**
 * @param {UserContextData} userCtx
 * @returns {Promise<Club[]>}
 */
export async function loadMyClubs(userCtx) {
    const response = await userCtx.authenticatedGet("/clubs/joined");
    /** @type { {clubs: Club[]} } */
    const data = await response?.json();
    return data.clubs;
}

/**
 * Return a list of shallow club information.
 *
 * @returns {Promise<Club[]>}
 */
export async function loadPublicClubs(fetch) {
    const response = await apiRequest("/clubs", {}, fetch);
    if (response.okResponse) {
        /** @type { {clubs: Club[]} } */
        const data = await response.okResponse.json();
        return data.clubs;
    } else {
        console.warn("loading public clubs failed", response.error, response.errorBody);
        return [];
    }
}

/**
 * @param {number|string} clubId 
 * @param {UserContextData} [userCtx] 
 * @param {(input: any) => Promise<Response>} [svelteFetch]
 */
export async function loadAndSetClubDetails(clubId, userCtx, svelteFetch) {
    const details = await loadClubDetails(clubId, userCtx, svelteFetch);
    clubsData.currentClubDetails = details;
    clubsData.loadedForUser = true;
}

/**
 * @param {number|string} clubId 
 * @param {UserContextData} [userCtx] 
 * @param {(input: any) => Promise<Response>} [svelteFetch]
 * @returns {Promise<ClubDetailsResponse>}
 */
export async function loadClubDetails(clubId, userCtx, svelteFetch) {
    const resolvedFetch = svelteFetch || fetch;
    let response;
    if (userCtx) {
        response = await userCtx.authenticatedGet(`/clubs/${clubId}/private`);
    } else {
        response = await resolvedFetch(`${PUBLIC_API_BASE}/clubs/${clubId}`);
    }

    if (!response?.ok) {
        console.error("failed fetching data", response);
        error(502, 'bad gateway');
    }
    /** 
    * @type {ClubDetailsResponse}
    */
    return await response.json();
}

/**
 * @param {UserContextData} userCtx
 * @param {string} title
 * @param {string} description
 * @param {string} [url]
 * @returns {Promise<Club|undefined>}
 */
export async function createNewClub(userCtx, title, description, url) {
    if (title.length > 64 || description.length > 1024 || (url && url.length > 255)) {
        // UI should catch this!
        throw new Error("club title, url, or description too long");
    }

    if (url?.length === 0) {
        url = undefined
    };
    if (url) {
        url = validateUrl(url);
    }

    const response = await userCtx.authenticatedPost("/clubs/create", { title, description, url });

    if (response?.status !== 201) {
        console.error("Failed to create club", response);
        return;
    }

    /** @type { Club } */
    const club = await response?.json();
    clubsData.mine.push(club);
    return club;
}

/**
 * @param {UserContextData} userCtx
 * @param {number} clubId
 * @param {Blob} blob
 */
export async function updateClubAvatar(userCtx, clubId, blob) {

    const formData = new FormData();
    formData.append('avatar', blob, 'avatar.png');

    const headers = {};
    const response = await userCtx.authenticatedApiRequest("POST", `/clubs/${clubId}/avatar`, headers, formData);

    if (response.error) {
        console.error("Failed to update avatar", response.error, response.errorBody);
        return;
    }
}

/**
 * @param {UserContextData} userCtx
 * @param {number} clubId
 * @param {EditableClubDetails} details
 */
export async function updateClub(userCtx, clubId, details) {

    const response = await userCtx.authenticatedPost(`/clubs/${clubId}`, details);

    if (!response?.ok) {
        console.error("Failed to update club", response);
        return;
    }

    const index = clubsData.mine.findIndex((club) => club.id == clubId);
    if (index != -1) {
        clubsData.mine[index].description = details.description;
    }
}

/**
 * @param {UserContextData} userCtx
 * @param {number} clubId
 * @param {string} name
 * @param {string} description
 * @param {boolean} isPublic
 * @returns {Promise<{playlist_id: number} | undefined>}
 */
export async function createNewClubPlaylist(userCtx, clubId, name, description, isPublic) {
    const body = {
        display_name: name,
        description,
        public: isPublic,
    }
    const response = await userCtx.authenticatedPost(`/clubs/${clubId}/playlist/new`, body);

    if (!response?.ok) {
        console.error("Failed to create playlist", response);
        return;
    }

    return await response.json();
}

/**
 * @param {string} input
 * @returns {string}
 */
function validateUrl(input) {
    let prefixedUrl = input;

    if (!input.startsWith("http")) {
        prefixedUrl = "https://" + input;
    }

    // throws an error if it's an invalid url
    const url = new URL(prefixedUrl);
    return url.href;
}