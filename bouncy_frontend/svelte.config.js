import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter from "svelte-adapter-bun";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({ exposeStats: true }),
    paths: {
      base: process.env.NODE_ENV === 'development' ? '' : '',
    }
  },

  preprocess: [vitePreprocess({})]
};

export default config;
