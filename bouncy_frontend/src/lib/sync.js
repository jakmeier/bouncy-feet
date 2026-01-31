// Locally store modifications and sync them with peers and / or central
// data authorities.
//
// Note: this module is a candidate to become a WASM component in the future

import { browser } from "$app/environment";

/**
 * @callback UpdateRemoteCallback
 * @param {string} typedKey
 * @param {string} value
 * @param {Date} lastModified
 * @param {number} version
 * @returns {Promise<void>}
 */

/**
 * @callback OnLocalUpdatedCallback
 * @param {string} typedKey
 * @param {string} value
 * @param {string} type
 * @param {Date} lastModified
 * @param {number} version
 * @returns {Promise<void>}
 */

/** Value type prefixes */
const PREFIX_STRING_T = "s:";
const PREFIX_NUMBER_T = "n:";

/** Namespace for managing the KV store itself. */
const NS_MANAGEMENT = "M_";

/**
 * @typedef {string} SyncMethod
 *
 * @readonly
 * @enum {SyncMethod} 
 */
const SYNC_METHOD = {
    // Set the value to a new value, overwriting old values.
    SET: "SET",
}

/**
 * @typedef {Object} KvModification
 * 
 * @property {SyncMethod} method
 * @property {string} v -- value
 * @property {string} t -- timestamp, iso string of date
 * @property {string} ver -- version
 */

/**
 * Keep track of client-side changes to a key-value store and manages
 * synchronization with a remote.
 */
export class KvSync {
    /**
     * @param {string} localStorageKeyPrefix
     * @param {UpdateRemoteCallback} updateRemote
     * @param {OnLocalUpdatedCallback} onUpdatedLocal
     */
    constructor(localStorageKeyPrefix, updateRemote, onUpdatedLocal) {
        if (!browser) {
            return;
        }

        /** @type {UpdateRemoteCallback} */
        this.updateRemote = updateRemote;

        /** @type {OnLocalUpdatedCallback} */
        this.onUpdatedLocal = onUpdatedLocal;

        // The only version used right now.
        // The idea is that different versions can change type requirements on keys.
        /** @type {number} */
        this.version = 0;

        // All local data modifications are done directly on localstorage
        this.prefix = localStorageKeyPrefix;

        const localVersion = this.#getLocalNumber("version", NS_MANAGEMENT);
        if (localVersion === undefined) {
            const namespace = NS_MANAGEMENT;
            const type = PREFIX_NUMBER_T;
            const key = "version";
            const typedKey = `${namespace}${type}${key}`;
            // Update without triggering update on the remote, set value as
            // older than anything on the remote
            const date = new Date(2000, 0, 0, 0, 0);
            this.#setLocalValueSideEffectFree(typedKey, this.version.toString(), date);
        }

        const localVersionAfter = this.#getLocalNumber("version", NS_MANAGEMENT);
        if (localVersionAfter !== this.version) {
            throw new Error(`Unsupported version in key-value store ${localVersionAfter}`);
        }
    }

    /**
     * Sets a local string value and records its metadata.
     * 
     * Triggers remote updates if necessary.
     *
     * @param {string} key
     * @param {string} value
     * @param {Date} lastModified
     * @returns {Promise<boolean>}
     */
    async setStringValue(key, value, lastModified) {
        const updated = await this.#setLocalValue(key, value, lastModified, PREFIX_STRING_T);
        return updated;
    }

    /**
     * Synchronizes local data with the given remote snapshot.
     *
     * @param {UserMetaResponse[]} remoteData
     * @returns {Promise<void>}
     */
    async sync(remoteData) {
        // Step 1: 2-way sync all keys the remote knows
        // (and build a set of remote-keys for step 2)
        const remoteKeys = new Set();
        for (const item of remoteData) {
            const typedKey = item.key_name;
            remoteKeys.add(typedKey);

            const remoteTimestamp = item.last_modified ? new Date(item.last_modified) : new Date();
            const localMods = this.#getModifications(typedKey);
            if (localMods.length === 0) {
                // Data from the remote is new
                this.#setLocalValueSideEffectFree(typedKey, item.key_value, remoteTimestamp);
                continue;
            }

            // With only SET method, we only ever need one modification.
            // Once ADD etc are implemented, this needs to be more complicated.
            const lastLocalMod = localMods[localMods.length - 1];
            const localTimestamp = new Date(lastLocalMod.t);
            if (localTimestamp < remoteTimestamp) {
                // Data from the remote is newer than local data
                this.#setLocalValueSideEffectFree(typedKey, item.key_value, remoteTimestamp);
                continue;
            }
        }

        // Step 2: Sync back values missing on the remote
        const localKeys = this.#allLocalKeys();
        const localOnlyKeys = localKeys.difference(remoteKeys);
        const promises = [];
        for (const typedKey of localOnlyKeys) {
            const localMods = this.#getModifications(typedKey);
            // With only SET method, we only ever need one modification.
            // Once ADD etc are implemented, this needs to be more complicated.
            const lastLocalMod = localMods[localMods.length - 1];
            const promise = this.updateRemote(typedKey, lastLocalMod.v, new Date(lastLocalMod.t), this.version);
            promises.push(promise);
        }

        await Promise.all(promises);

    }

    /** @returns {DynUserMeta} */
    load() {
        /** @type {DynUserMeta} */
        const out = {};
        for (const typedKey of this.#allLocalKeys()) {
            // split once
            var i = typedKey.indexOf(':');
            if (i !== -1) {
                const prefix = typedKey.slice(0, i + 1);
                const key = typedKey.slice(i + 1);
                const value = this.#getLocalValue(key, prefix);
                if (value) {
                    out[key] = value;
                }
            }
        }
        return out;
    }

    /**
     * Returns true if the value was updated or false if a newer value was
     * already stored.
     *
     * If this method updates the value, it also triggers a remote update and
     * calls onUpdatedLocal.
     *
     * @param {string} key
     * @param {string} value
     * @param {Date} timestamp
     * @param {string} type
     * @param {string} [namespace]
     * @returns {Promise<boolean>}
    */
    async #setLocalValue(key, value, timestamp, type, namespace = "") {
        const typedKey = `${namespace}${type}${key}`;

        const oldMods = this.#getModifications(typedKey);
        if (oldMods.length > 0 && new Date(oldMods[oldMods.length - 1].t) > timestamp) {
            // existing value is newer!
            return false;
        }

        this.#setLocalValueSideEffectFree(typedKey, value, timestamp);
        this.onUpdatedLocal(key, value, type, timestamp, this.version);
        await this.updateRemote(typedKey, value, timestamp, this.version);

        return true;
    }

    /**
     * Directly writes a SET modification locally, no checks, no remote updates.
     *
     * @param {string} typedKey
     * @param {string} value
     * @param {Date} timestamp
    */
    #setLocalValueSideEffectFree(typedKey, value, timestamp) {
        if (!browser) {
            return;
        }
        /** @type {KvModification} */
        const mod = {
            method: SYNC_METHOD.SET,
            v: value,
            t: timestamp.toISOString(),
            ver: this.version.toString()
        };
        // With only SET method, we only ever need one modification, so we can
        // ignore old values and replace it with the latest SET.
        // Once ADD etc are implemented, this needs to be more complicated.
        localStorage.setItem(this.prefix + typedKey, JSON.stringify([mod]));
    }

    /**
     * Returns true if the value was updated or false if a newer value was already stored.
     *
     * @param {string} key
     * @param {number} value
     * @param {Date} lastModified
     * @param {string} [namespace]
     * @returns {Promise<boolean>}
    */
    async #setLocalNumber(key, value, lastModified, namespace = "") {
        return this.#setLocalValue(key, value.toString(10), lastModified, PREFIX_NUMBER_T, namespace);
    }

    /**
     * Read the latest value derived from all known modifications.
     *
     * @param {string} key
     * @param {string} type
     * @param {string} [namespace]
     * @returns {string|undefined}
    */
    #getLocalValue(key, type, namespace = "") {
        const typedKey = `${namespace}${type}${key}`;
        const mods = this.#getModifications(typedKey);

        if (mods.length === 0) {
            return undefined;
        }

        // With only SET method, simply take the latest value.
        // Once ADD etc are implemented, this needs to be more complicated.
        return mods[mods.length - 1].v;
    }

    /**
     * @param {string} key
     * @param {string} [namespace]
     * @returns {number|undefined}
    */
    #getLocalNumber(key, namespace = "") {
        const raw = this.#getLocalValue(key, PREFIX_NUMBER_T, namespace);
        const num = Number(raw);
        if (Number.isNaN(num)) {
            return undefined;
        }
        return num;
    }

    /**
     * @param {string} key
     * @param {string} [namespace]
     * @returns {string | undefined}
    */
    #getLocalString(key, namespace = "") {
        return this.#getLocalValue(key, PREFIX_STRING_T, namespace);
    }

    /**
     * @param {string} typedKey
     * @returns {KvModification[]}
    */
    #getModifications(typedKey) {
        if (!browser) {
            return [];
        }
        const raw = localStorage.getItem(this.prefix + typedKey);
        if (raw === null) {
            return [];
        }
        const mods = JSON.parse(raw);
        if (Array.isArray(mods)) {
            return mods;
        }
        return [];
    }

    /**
     * Returns all _typed_ keys but without the local storage prefix.
     * 
     * @returns {Set<string>}
     */
    #allLocalKeys() {
        if (!browser) {
            return new Set();
        }
        const out = new Set();
        for (let i = 0; i < localStorage.length; i++) {
            const key = localStorage.key(i);
            if (key?.startsWith(this.prefix)) {
                out.add(key.slice(this.prefix.length));
            }
        }
        return out;
    }
}
