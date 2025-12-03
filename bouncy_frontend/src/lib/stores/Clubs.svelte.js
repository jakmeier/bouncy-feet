import { apiRequest } from "$lib/stats";
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
 * @returns {Promise<Club[]>}
 */
export async function loadPublicClubs() {
    const response = await apiRequest("/clubs");
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
 * @param {UserContextData} userCtx
 * @param {string} title
 * @param {string} description
 */
export async function createNewClub(userCtx, title, description) {
    if (title.length > 64 || description.length > 1024) {
        // UI should catch this!
        throw new Error("club title or description too long");
    }

    const response = await userCtx.authenticatedPost("/clubs/create", { title, description });

    if (response?.status !== 201) {
        console.error("Failed to create club", response);
        return;
    }

    /** @type { Club } */
    const club = await response?.json();
    clubsData.mine.push(club);
}