/**
 * The service worker adds offline capabilities and improves performance.
 * 
 * It works by intercepting all network requests and caching responses through
 * the browser Cache API. For static assets, we only go to the network the first
 * time, which improves loading time thereafter. For all requests, if the device
 * is offline, the service worker provides the most recent response from when it
 * was online the last time.
 * 
 * If specific data must never be caches, this has to be special-cased in here.
 */

/// <reference types="@sveltejs/kit" />
import { build, files, version } from '$service-worker';

// Create a unique cache name for this deployment
const VERSIONED_CACHE = `cache-${version}`;

const ASSETS = [
    ...build, // the app itself
    ...files  // everything in `static`
];

const CDNS = [
    "cdn.jsdelivr.net",
];

self.addEventListener("install", (event) => {
    // Create a new cache and add all files to it
    async function addFilesToCache() {
        const cache = await caches.open(VERSIONED_CACHE);
        await cache.addAll(ASSETS);
    }

    event.waitUntil(addFilesToCache());
});

self.addEventListener("activate", (event) => {
    // Remove previous cached data from disk
    async function deleteOldCaches() {
        for (const key of await caches.keys()) {
            if (key !== VERSIONED_CACHE) await caches.delete(key);
        }
    }

    event.waitUntil(deleteOldCaches());
});

self.addEventListener("fetch", (event) => {
    // only use cache for GET requests
    if (event.request.method !== "GET") return;

    async function respond() {
        const url = new URL(event.request.url);
        const cache = await caches.open(VERSIONED_CACHE);

        // static files are offline first
        const isStatic = ASSETS.includes(url.pathname) || CDNS.includes(url.hostname);
        if (isStatic) {
            return cache.match(url.pathname);
        }


        // for everything else, network first, only use he cache when offline
        try {
            const response = await fetch(event.request);

            if (response.status === 200) {
                cache.put(event.request, response.clone());
            }

            return response;
        } catch {
            return cache.match(event.request);
        }
    }

    event.respondWith(respond());
});