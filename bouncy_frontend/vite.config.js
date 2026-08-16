import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { fileURLToPath } from 'node:url';
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
    plugins: [
        sveltekit(),
        VitePWA({
            strategies: 'generateSW',
            workbox: {
                // Precache app shell + hashed assets
                globPatterns: ['**/*.{js,css,html,wasm}'],
                maximumFileSizeToCacheInBytes: 5 * 1024 * 1024, // 5 MiB
            },
        })
    ],
    resolve: {
        // The PeerTube submodule does not commit this package's generated dist/ files.
        // Bundle its source directly so clean CI checkouts do not need a PeerTube build.
        alias: {
            '@peertube/embed-api': fileURLToPath(new URL('../bouncy_tube/PeerTube/client/src/standalone/embed-player-api/player.ts', import.meta.url))
        }
    },
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    },
    ssr: {
        noExternal: ['@mediapipe/tasks-vision']
    }
});
