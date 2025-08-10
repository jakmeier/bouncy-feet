
/**
 * @typedef {Object} AccessToken
 * @property {string} token_type
 * @property {string} access_token
 * @property {string} refresh_token
 * 
 * @typedef {Object} PwaAuth
 * @property {boolean} isAuthenticated
 * @property {null|import('keycloak-js').default} keycloakInstance
 * @property {null|import('keycloak-js').KeycloakProfile} userProfile
 * @property {null|AccessToken} peerTubeToken
 */

// Pwa auth works independent of the api server, generating a token to be used
// for PeerTube only (for now).
/** @type {PwaAuth} */
export const pwaAuth = $state({
    isAuthenticated: false,
    keycloakInstance: null,
    userProfile: null,
    peerTubeToken: null,
});
