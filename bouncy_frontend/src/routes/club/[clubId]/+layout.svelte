<script>
  import { page } from '$app/state';
  import { clubsData } from '$lib/stores/Clubs.svelte';
  import { onMount } from 'svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import LoadClubInfo from './LoadClubInfo.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').LayoutData} data
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { data, children } = $props();

  const clubId = Number.parseInt(page.params.clubId || '0');
  // May be false while clubs are still loading and later becomes true.
  /** @type {boolean} */
  const isMember = $derived.by(
    () => clubsData.mine.findIndex((c) => c.id === clubId) !== -1
  );

  onMount(() => {
    clubsData.currentClubDetails = data.publicClubDetails;
  });
</script>

<LoginRequiredContent
  >{#snippet guest({ apiUser })}
    <LoadClubInfo {apiUser} {isMember} clubId />
    {@render children?.()}
  {/snippet}
</LoginRequiredContent>
