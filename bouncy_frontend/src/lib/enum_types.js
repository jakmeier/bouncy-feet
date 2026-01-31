/**
 * @typedef {number} UserAuthState
 * | User Type                          | API User | OpenID | PeerTube Access Token |
 * |------------------------------------|----------|--------|-----------------------|
 * | Anonymous                          | no       | no     | no                    |
 * | Guest                              | yes      | no     | no                    |
 * | SignedUpUserExpiredAPISession      | no       | yes    | no                    |
 * | SignedUpUserExpiredPeerTubeSession | yes      | yes    | no                    |
 * | SignedUpUser                       | yes      | yes    | yes                   |
 * @enum {UserAuthState}
 *
 */
export const USER_AUH_STATE = {
    Anonymous: 0,
    Guest: 1,
    SignedUpUserExpiredAPISession: 2,
    SignedUpUserExpiredPeerTubeSession: 3,
    SignedUpUser: 4,
};