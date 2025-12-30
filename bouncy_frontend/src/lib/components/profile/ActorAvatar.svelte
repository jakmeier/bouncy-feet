<script>
  import { resolve } from '$app/paths';
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import * as api from '$lib/peertube-openapi';

  /**
   * @typedef {Object} Props
   * @property {api.Actor} [actor]
   * @property {string} [url]
   * @property {string} [defaultSymbol]
   * @property {number} [targetWidth]
   */

  /** @type {Props} */
  let {
    actor,
    defaultSymbol = '/img/symbols/bf_club.svg',
    targetWidth = 192,
    url,
  } = $props();

  const logoPath = $derived.by(() => {
    if (url) {
      return url;
    }
    if (actor && actor.avatars && actor.avatars.length >= 1) {
      let i = 0;
      while (
        i + 1 < actor.avatars.length &&
        actor.avatars[i].width < targetWidth
      ) {
        i += 1;
      }
      return PUBLIC_BF_PEERTUBE_URL + actor.avatars[i].path;
    } else {
      return resolve(defaultSymbol);
    }
  });
</script>

<img
  class="avatar"
  src={logoPath}
  alt="club avatar"
  style:max-width="{targetWidth}px"
/>

<style>
  .avatar {
    border-radius: 1rem;
    width: 100%;
    border: solid var(--theme-main-medium) 0.25rem;
    background-color: var(--theme-main-alt);
  }
</style>
