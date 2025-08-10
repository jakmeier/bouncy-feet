import Keycloak from 'keycloak-js';
import { pwaAuth } from '$lib/stores/Auth.svelte';

// TODO(July): env specific urls
// TODO(July): fix constant reloads and warnings in console before first login
const pwaKeycloak = new Keycloak({
    url: 'https://auth.bouncy-feet.ch/',
    realm: 'bouncyfeet-dev',
    clientId: 'dev-pwa'
});

export async function initKeycloakAuth() {
    const authenticated = await pwaKeycloak.init({
        onLoad: 'check-sso',
        silentCheckSsoRedirectUri: `${window.location.origin}/silent-check-sso.html`,
        pkceMethod: 'S256'
    });

    pwaAuth.isAuthenticated = authenticated;
    pwaAuth.keycloakInstance = pwaKeycloak;

    if (authenticated) {
        const profile = await pwaKeycloak.loadUserProfile();
        pwaAuth.userProfile = profile;
    }

    // Optional: auto-refresh token
    setInterval(() => {
        pwaKeycloak.updateToken(30).catch(() => {
            pwaKeycloak.logout();
        });
    }, 10000);

    return authenticated;
}

export function triggerLogin() {
    pwaKeycloak.login();
}

export function triggerLogout() {
    pwaKeycloak.logout();
}

export function getToken() {
    return pwaKeycloak.token;
}

export async function fetchWithAuth(url, options = {}) {
    await pwaKeycloak.updateToken(30); // refresh if about to expire
    const token = pwaKeycloak.token;

    return fetch(url, {
        ...options,
        headers: {
            ...(options.headers || {}),
            Authorization: `Bearer ${token}`
        }
    });
}
