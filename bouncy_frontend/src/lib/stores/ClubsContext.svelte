<script>
  import { onMount, setContext } from 'svelte';
  import { clubsData, loadMyClubs } from './Clubs.svelte';
  import { getUserContext } from '$lib/context';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /** @type {ClubsContextData} */
  const ctx = { clubsData };
  setContext('clubs', ctx);

  const userCtx = getUserContext();

  onMount(async () => {
    await loadMyClubs(userCtx);
  });
</script>

{@render children?.()}
