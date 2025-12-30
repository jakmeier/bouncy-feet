<script>
  import { page } from '$app/state';
  import { getUserContext } from '$lib/context';
  import { clubsData, loadAndSetClubDetails } from '$lib/stores/Clubs.svelte';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').LayoutData} data
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { data, children } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();
  const clubId = Number.parseInt(page.params.clubId || '0');
  // May be false while clubs are still loading and later become true.
  /** @type {boolean} */
  const isMember = $derived.by(
    () => clubsData.mine.findIndex((c) => c.id === clubId) !== -1
  );

  onMount(() => {
    clubsData.currentClubDetails = data.publicClubDetails;

    // Read club details only available to members
    function triggerLater() {
      if (isMember) {
        loadAndSetClubDetails(clubId, userCtx);
      } else {
        setTimeout(triggerLater, 50);
      }
    }
    triggerLater();
  });
</script>

{@render children?.()}
