import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public'
import { error } from '@sveltejs/kit';
import * as api from '$lib/peertube-openapi';
import { apiRequest } from '$lib/stats';

/** @type {import('@sveltejs/kit').Load} */
export const load = async ({ fetch, params }) => {
    if (typeof params.userId !== "string") {
        error(404, 'no user selected');
    }
    const response = await apiRequest(`/users/${params.userId}`, {}, fetch);
    if (!response.okResponse) {
        console.error('failed fetching user', response.error, response.errorBody);
        error(500, response.error);
    }
    /** @type {PublicUserResponse} */
    const displayedUser = await response.okResponse.json();

    /** @type {api.VideoChannel | null} */
    let account = null;
    // could use api.foo() but that wouldn't use svelte's `fetch`
    if (displayedUser.peertube_handle) {
        const accountResponse = await fetch(`${PUBLIC_BF_PEERTUBE_URL}/api/v1/accounts/${displayedUser.peertube_handle}`);
        if (!accountResponse.ok) {
            console.error('failed fetching account data', accountResponse);
            error(502, 'bad gateway');
        }

        /** @type {api.Account} */
        account = await accountResponse.json();
    }

    return {
        /** @type {PublicUserResponse & { account: api.Account | null } } */
        displayedUser: {
            ...displayedUser,
            account,
        }
    }
}