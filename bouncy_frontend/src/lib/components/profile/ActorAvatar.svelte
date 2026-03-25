<script>
  import { resolve } from '$app/paths';
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import * as api from '$lib/peertube-openapi';
  import BfGenLogo from '$lib/components/ui/svg/BfGenLogo.svelte';

  /**
   * @typedef {Object} Props
   * @property {api.Actor} [actor]
   * @property {string} [url]
   * @property {string} [defaultSymbol]
   * @property {number} [targetWidth]
   * @property {boolean} [isClub]
   * @property {number} seed
   */

  /** @type {Props} */
  let {
    actor,
    defaultSymbol = undefined,
    targetWidth = 192,
    url,
    isClub = false,
    seed,
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
    } else if (defaultSymbol) {
      return resolve(defaultSymbol);
    } else {
      return null;
    }
  });
</script>

{#if logoPath}
  <img
    class="avatar"
    src={logoPath}
    alt="club avatar"
    style:max-width="{targetWidth}px"
  />
{:else}
  <div
    class="avatar"
    style:width="{targetWidth}px"
    style:height="{targetWidth}px"
  >
    <BfGenLogo
      {isClub}
      seed={isClub ? seed + 100_000_000 : seed}
      size={targetWidth}
    ></BfGenLogo>
  </div>
{/if}

<style>
  .avatar {
    border-radius: 1rem;
    width: 100%;
    height: 100%;
    border: solid var(--theme-main-medium) 0.25rem;
    margin: -0.25rem;
    background-color: var(--theme-main-alt);
    overflow: hidden;
  }
</style>
