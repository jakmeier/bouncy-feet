/**
 * A client session is for guest and full users.
 *
 * It provides
 *
 * - authenticated requests to the API server
 * - user local storage that is synced with a KV store on the API server
 *
 */

import { ClientSession } from './ClientSession.svelte';
import { API_ERROR, apiRequest, requestNewGuestSession } from '$lib/stats';
import { KvSync } from "$lib/sync";
import { DetectionResult } from '$lib/instructor/bouncy_instructor';

export class ApiUser {
    /**
     * @param {UserContextData} userCtx
     * @param {ClientSession} clientSession
     */
    constructor(userCtx, clientSession) {
        this.userCtx = userCtx;
        this.clientSession = clientSession;

        this.kvSync = new KvSync('bfkv_', this.updateMetaOnRemote.bind(this), this.updateMetaInMemory.bind(this));
        this.meta = this.kvSync.load();
    }

    /**
     * Restore an existing API User from local storage. Optionally creates a
     * new guest client session.
     * 
     * The returned API user does not necessarily have an active login session
     * with the API, yet!
     *
     * @param {boolean} createGuest
     * @param {UserContextData} userCtx
     * @returns {Promise<ApiUser | undefined>}
     */
    static async initApiUser(createGuest, userCtx) {
        const clientSession = await ClientSession.initClientSession(createGuest);
        if (!clientSession) {
            return;
        }

        const apiUser = new ApiUser(userCtx, clientSession);

        return apiUser;
    }

    /**
     * @param {string} path
     * @returns {Promise<import('$lib/stats').ApiResponse>}
     */
    async authenticatedGet(path) {
        return await this.authenticatedApiRequest('GET', path, {}, undefined);
    }

    /**
     * @param {string} path
     * @param {object} body
     * @returns {Promise<Response | null | undefined>}
     */
    async authenticatedPost(path, body) {
        const headers = {
            'Content-Type': 'application/json',
        };
        const result = await this.authenticatedApiRequest(
            'POST',
            path,
            headers,
            JSON.stringify(body)
        );
        if (result?.okResponse) {
            return result.okResponse;
        }
        console.warn('post failed', result);
    }

    /**
     * @param {string} method
     * @param {string} path
     * @param {object} headers
     * @param {string|FormData|undefined} body
     * @returns {Promise<import('$lib/stats').ApiResponse>}
     */
    async authenticatedApiRequest(method, path, headers, body) {
        let auth = this.#authHeader();
        const options = {
            method,
            headers: {
                ...headers,
                ...auth,
            },
            body,
        };

        const result = await apiRequest(path, options);

        if (result.error || result.errorBody) {
            switch (result.error) {
                case API_ERROR.NeedLogin: {
                    // bubble up, RequiresLogin should handle this
                    break;
                }
                case API_ERROR.UserNotFound: {
                    // An older version may have lost the client session server-side. Use
                    // local client state to recreate it with a new client session.
                    // TODO: Remove this after a few updates, as this shouldn't be necessary
                    // with the new code working.

                    // try to get a new session
                    const newSession = await requestNewGuestSession();

                    // replace local session id and secret but keep other fields
                    console.warn('replacing old session with a new');
                    localStorage.clientSessionId = newSession.client_session_id;
                    localStorage.clientSessionSecret = newSession.client_session_secret;
                    this.clientSession.clientSessionData.id = newSession.client_session_id;
                    this.clientSession.clientSessionData.secret = newSession.client_session_secret;

                    // now update the server about our local state
                    let err = this.syncKvWithServer();
                    if (err) {
                        console.error("Failed syncing", err);
                    }

                    // now we can try to make the unauthorized request again, with new auth headers
                    let auth = this.#authHeader();
                    const options = {
                        method,
                        headers: {
                            ...headers,
                            ...auth,
                        },
                        body,
                    };
                    return await apiRequest(path, options);
                }
                case API_ERROR.ClientSessionLoginNotAllow: {
                    // Must use keycloak login.
                    // This shouldn't happen but if it does, we could trigger a refresh of user info.
                    console.warn('Tried logging in as guest while being a full user.');
                }
                case API_ERROR.ClientSessionOfDifferentUser: {
                    // TODO: Need user input to resolve.
                    // Did the user mean to login to a different account?
                    // What happens to locally stored data?
                    // => Show $user.publicName and ask: log out and change user?
                    // => About deleting:
                    // - just user data and userMeta needs deleting, which should be
                    //   synced to the sever. If a sync is pending, tell the user before
                    //   they log out!
                    // - steps / poses don't need to be deleted, can be kept locally
                    // - localState can probably be kept, too
                    break;
                }
                default:
                    throw result;
            }
        }

        return result;
    }

    /** @returns {Promise<import('$lib/stats').ApiError | undefined>} */
    async syncKvWithServer() {
        const remoteData = await this.authenticatedGet('/user/meta');
        if (remoteData && remoteData.okResponse) {
            const remoteMods = await remoteData.okResponse.json();
            if (Array.isArray(remoteMods)) {
                await this.kvSync.sync(remoteMods);
            } else {
                console.error('Unexpected response from meta query', remoteMods);
            }
        } else if (remoteData.error === API_ERROR.NeedLogin) {
            return API_ERROR.NeedLogin;
        }
        else {
            console.error('Meta query failed', remoteData);
        }
    }

    /**
     * @param {string} key
     * @param {string} value
     * @returns {Promise<void>}
     */
    async setUserMeta(key, value) {
        if (!key) {
            console.warn('Tried setting user meta with empty key');
            return;
        }

        // All updates go through sync object which first persists in local storage
        // and then updates in-memory and remote states.
        await this.kvSync.setStringValue(key, value, new Date());

        return;
    }

    /**
     * Read all user meta from the backend server.
     *
     * @returns {Promise<UserMetaResponse[] | undefined>}
     */
    async getAllUserMeta() {
        const response = await this.authenticatedGet('/user/meta');

        if (response.okResponse) {
            return response.okResponse.json();
        }
    }

    skippedIntro() { return hasSkippedIntro }

    /** @param {boolean} yes */
    setSkippedIntro(yes) {
        hasSkippedIntro = yes;
    }

    /**
     * @param {string} activityId
     * @param {DanceSessionResult} sessionResult
     */
    async submitActivityCall(activityId, sessionResult) {
        const headers = {
            'Content-Type': 'application/json',
        };
        const body = JSON.stringify({
            client_session_id: Number.parseInt(this.clientSession.clientSessionData.id),
            client_session_secret: this.clientSession.clientSessionData.secret,

            activity_id: activityId,
            recorded_at: sessionResult.timestamp.toISOString(),
            hits: sessionResult.hits,
            misses: sessionResult.misses,
            steps: sessionResult.numSteps,
        });

        return await this.authenticatedApiRequest(
            'POST',
            '/new_guest_activity',
            headers,
            body
        );
    }

    /**
     * @param {string} courseId
     * @param {number} lessonIndex
     * @param {DetectionResult} detection
     *
     * @returns {DanceSessionResult?}
     */
    submitCourseLesson(courseId, lessonIndex, detection) {
        const id = `course/${courseId}[${lessonIndex}]`;
        return this.submitActivity(id, detection);
    }

    /**
     * @param {string} warmupId
     * @param {DetectionResult} detection
     *
     * @returns {DanceSessionResult?}
     */
    submitWarmup(warmupId, detection) {
        const id = `warmup/${warmupId}`;
        return this.submitActivity(id, detection);
    }

    /**
     * @param {string} stepId
     * @param {number} bpm
     * @param {DetectionResult} detection
     *
     * @returns {DanceSessionResult?}
     */
    submitStepTraining(stepId, bpm, detection) {
        const id = `step/${stepId}[${bpm}]`;
        return this.submitActivity(id, detection);
    }

    /**
     * @param {string} activityId
     * @param {DetectionResult} detection
     *
     * @returns {DanceSessionResult?}
     */
    submitActivity(activityId, detection) {
        const limitedId = activityId.slice(0, 256);
        const hits = detection.poseMatches;
        const misses = detection.poseMisses;
        const steps = detection.steps();
        const numSteps = steps.length;
        const duration =
            steps.length > 0
                ? Number(steps[steps.length - 1].end - steps[0].start)
                : 0;

        /** @type {DanceSessionResult} */
        const sessionResult = {
            numSteps,
            hits,
            misses,
            duration,
            timestamp: new Date(),
        };

        try {
            // not awaiting here, it can happen in the background
            this.submitActivityCall(limitedId, sessionResult);
        } catch (err) {
            console.warn('Submitting activity stats failed', err);
        }
        // TODO: store locally

        return sessionResult;
    }

    // Update local stats, offline only.
    /**
     * @param {DanceSessionResult} result
     */
    addDanceToStats(result) {
        this.userCtx.user.recordedDances += 1;
        this.userCtx.user.recordedSteps += result.numSteps;
        this.userCtx.user.recordedSeconds += result.duration / 1000;
    }

    /**
     * @param {string} key
     * @param {string} value
     * @param {Date} lastModified
     * @param {number} version
     * @returns {Promise<void>}
     */
    async updateMetaOnRemote(key, value, lastModified, version) {
        const headers = {
            'Content-Type': 'application/json',
        };
        const body = JSON.stringify({
            key_name: key,
            key_value: value,
            last_modified: lastModified,
            version,
        });

        const result = await this.authenticatedApiRequest(
            'POST',
            '/user/meta/update',
            headers,
            body
        );

        if (result.error || result.errorBody) {
            // TODO: handle login etc
            console.warn('meta update failed', result.errorBody);
        }

        return;
    }

    /**
     * @param {string} key
     * @param {string} value
     * @param {string} type
     * @param {Date} _lastModified
     * @param {number} _version
     */
    async updateMetaInMemory(key, value, type, _lastModified, _version) {
        // for now, only handle strings here and avoid type conflicts
        // (the design might require some more iterations)
        if (type === 's:') {
            this.meta[key] = value;
        }
    }

    #authHeader() {
        if (this.userCtx.user.openid) {
            // The user is not a guest, so shouldn't use client session login.
            // Instead, use cookie session.
            return {};
        }
        return this.clientSession.authHeader();
    }

}

let hasSkippedIntro = $state(false);

