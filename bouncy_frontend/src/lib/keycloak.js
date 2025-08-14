import Keycloak from 'keycloak-js';
import { pwaAuth } from '$lib/stores/Auth.svelte';

// TODO(July): env specific realm / clientId
const pwaKeycloak = new Keycloak({
    url: 'https://auth.bouncy-feet.ch/',
    realm: 'bouncyfeet-dev',
    clientId: 'dev-pwa'
});

export async function initKeycloakAuth() {
    const authenticated = await pwaKeycloak.init({ pkceMethod: 'S256' });

    pwaAuth.isAuthenticated = authenticated;
    pwaAuth.keycloakInstance = pwaKeycloak;

    if (authenticated) {
        await afterLogin();
    }

    pwaKeycloak.onAuthSuccess = afterLogin;
    pwaKeycloak.onAuthError = afterLogout;
    pwaKeycloak.onAuthRefreshError = afterLogout;

    // Optional: auto-refresh token
    // Note: this triggers reload on every failed attempt
    // // at the very least, it should stop the interval after a fail and not even start if not authenticated
    // setInterval(() => {
    //     pwaKeycloak.updateToken(30).catch(() => {
    //         pwaKeycloak.logout();
    //     });
    // }, 20000);

    return authenticated;
}

async function afterLogin() {
    pwaAuth.userProfile = await pwaKeycloak.loadUserProfile();
    pwaAuth.isAuthenticated = pwaKeycloak.authenticated || false;
}

async function afterLogout() {
    pwaAuth.userProfile = null;
}

export function triggerLogin() {
    pwaKeycloak.login();
}

export function triggerRegister() {
    pwaKeycloak.register();
}

export function triggerLogout() {
    pwaKeycloak.logout();
}

export async function getToken() {
    if (pwaKeycloak.isTokenExpired(1)) {
        await pwaKeycloak.updateToken(30).catch(() => {
            pwaKeycloak.logout();
        });
    }
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
