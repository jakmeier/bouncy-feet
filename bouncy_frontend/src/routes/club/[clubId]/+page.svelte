<script>
  import { page } from '$app/state';
  import { getClubsContext } from '$lib/stores/Clubs.svelte';

  const clubId = Number.parseInt(page.params.clubId || '0');

  const { clubsData } = getClubsContext();
  const club = $derived(clubsData.mine.find((c) => c.id === clubId));
</script>

<h2>{club?.name}</h2>
<p>{club?.description}</p>
