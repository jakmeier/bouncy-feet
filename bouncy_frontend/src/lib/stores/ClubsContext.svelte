<script>
  import { onMount, setContext } from 'svelte';
  import { clubsData, loadMyClubs } from './Clubs.svelte';
  import { getUserContext } from '$lib/stores/context';
  import { USER_AUH_STATE } from '$lib/enum_types';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {Club[]} publicClubs
   */

  /** @type {Props} */
  let { children, publicClubs } = $props();

  /** @type {ClubsContextData} */
  const ctx = { clubsData };
  clubsData.public = publicClubs;
  setContext('clubs', ctx);

  const userCtx = getUserContext();

  // Note: It would be most sveltekit-like to load it on the server. But the
  // svelte server doesn't have the user credentials from userCtx. Thus, club
  // context is loaded empty first and dynamically filled in later. All pages
  // using clubs must handle this.
  // Blocking children to load doesn't work either, user interaction might be
  // necessary to log in and get clubs loaded.
  $effect(() => {
    if (userCtx.authState !== USER_AUH_STATE.Anonymous) {
      loadMyClubs(userCtx).then((clubs) => (clubsData.mine = clubs));
    }
  });
</script>

{@render children?.()}
