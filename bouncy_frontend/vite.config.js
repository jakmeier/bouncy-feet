import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
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
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    },
    ssr: {
        noExternal: ['@mediapipe/tasks-vision']
    }
});
