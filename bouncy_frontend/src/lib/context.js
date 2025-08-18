import { getContext } from "svelte";

/** @returns {UserContextData} */
export function getUserContext() {
    return getContext('user');
}
