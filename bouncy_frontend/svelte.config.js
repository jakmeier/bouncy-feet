import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from "@sveltejs/adapter-node";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter(),
    paths: {
      // base: process.env.NODE_ENV === 'development' ? '' : '',
      base: '',
    }
  },

  preprocess: [vitePreprocess({})]
};

export default config;
