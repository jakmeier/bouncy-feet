<script>
  import { clubsData, loadAndSetClubDetails } from '$lib/stores/Clubs.svelte';

  /**
   * @typedef {Object} Props
   * @property {boolean} isMember
   * @property {ApiUser} apiUser
   * @property {number} clubId
   */

  /** @type {Props} */
  let { isMember, apiUser, clubId } = $props();

  const _currentClubDetails = $derived.by(() => {
    if (isMember) {
      loadAndSetClubDetails(clubId, apiUser);
    } else {
      loadAndSetClubDetails(clubId);
    }
    return clubsData.currentClubDetails;
  });
</script>
