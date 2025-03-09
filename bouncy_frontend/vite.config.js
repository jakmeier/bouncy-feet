import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
// topLevelAwait makes problems in preview / production, since updating to svelte 5
// TypeError: Pyramid_0 is not a function
// import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm-esm";

export default defineConfig({
    plugins: [sveltekit(), wasm(["bouncy_instructor"])],
    // Without topLevelAwait, I need to set specific build targets
    // https://github.com/omnysecurity/vite-plugin-wasm-esm?tab=readme-ov-file
    build: {
        target: ["chrome89", "safari15", "firefox89"],
    },
    esbuild: {
        target: ["chrome89", "safari15", "firefox89"],
    },
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});
