<script>
  import { resolve } from '$app/paths';
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import * as api from '$lib/peertube-openapi';

  /**
   * @typedef {Object} Props
   * @property {api.Actor | null} actor
   * @property {string} [defaultSymbol]
   */

  /** @type {Props} */
  let { actor, defaultSymbol = '/img/symbols/bf_club.svg' } = $props();

  const logoPath = $derived.by(() => {
    if (actor && actor.avatars && actor.avatars.length >= 1) {
      return PUBLIC_BF_PEERTUBE_URL + actor.avatars[0].path;
    } else {
      return resolve(defaultSymbol);
    }
  });
</script>

<img class="avatar" src={logoPath} alt="club avatar" />

<style>
  .avatar {
    border-radius: 1rem;
    max-width: 192px;
    max-height: 192px;
    width: 100%;
    border: solid var(--theme-main-medium) 0.25rem;
  }
</style>
