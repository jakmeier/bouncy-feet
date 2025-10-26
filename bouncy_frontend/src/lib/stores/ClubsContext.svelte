<script>
  import { onMount, setContext } from 'svelte';
  import { clubsData, loadMyClubs } from './Clubs.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  setContext('clubs', {
    clubsData,
  });

  onMount(async () => {
    await loadMyClubs();
  });
</script>

{@render children?.()}
