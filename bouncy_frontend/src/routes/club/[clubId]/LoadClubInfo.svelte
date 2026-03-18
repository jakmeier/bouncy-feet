<script>
  import { clubsData, loadAndSetClubDetails } from '$lib/stores/Clubs.svelte';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {boolean} isMember
   * @property {ApiUser} apiUser
   * @property {number} clubId
   */

  /** @type {Props} */
  let { isMember, apiUser, clubId } = $props();

  onMount(async () => {
    if (isMember) {
      await loadAndSetClubDetails(clubId, apiUser);
    } else {
      await loadAndSetClubDetails(clubId);
    }
    return clubsData.currentClubDetails;
  });
</script>
