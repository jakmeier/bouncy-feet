import { vitePreprocess } from '@sveltejs/kit/vite';
// import adapter from '@sveltejs/adapter-static';
import adapter from "svelte-adapter-bun";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // This is for static hosting, which is no longer possible.
    // adapter: adapter({
    //   pages: 'build',
    //   assets: 'build',
    //   fallback: 'index.html',
    //   precompress: false,
    //   strict: true,
    //   prerender: {
    //     default: true,
    //   }
    // }),
    adapter: adapter(),
    paths: {
      base: process.env.NODE_ENV === 'development' ? '' : '',
    }
  },

  preprocess: [vitePreprocess({})]
};

export default config;
