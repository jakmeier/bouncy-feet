import { vitePreprocess } from '@sveltejs/kit/vite';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html',
      precompress: false,
      strict: true,
      prerender: {
        default: true,
      }
    }),
    paths: {
      base: process.env.NODE_ENV === 'development' ? '' : '',
    }
  },

  preprocess: [vitePreprocess({})]
};

export default config;
