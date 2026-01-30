import { getContext } from "svelte";

/** @returns {UserContextData} */
export function getUserContext() {
    return getContext('user');
}

/** @returns {ClubsContextData} */
export function getClubsContext() {
    const ctx = getContext('clubs');
    if (!ctx) {
        console.warn("missing clubs context");
    }
    return ctx;
}
