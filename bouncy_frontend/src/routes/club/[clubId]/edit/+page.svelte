<script>
  import { page } from '$app/state';
  import EditClub from '$lib/components/club/EditClub.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { getClubsContext } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const clubId = Number.parseInt(page.params.clubId || '0');
  const { clubsData } = getClubsContext();

  // May be undefined while clubs are still loading.
  /** @type {Club | undefined} */
  const club = $derived.by(
    () =>
      clubsData.mine.find((c) => c.id === clubId) ||
      clubsData.public.find((c) => c.id === clubId)
  );

  function onUpdateSuccess() {
    history.back();
  }
</script>

<LimeSection>
  <LogoHeader title={club?.name} backButton mainColor></LogoHeader>

  {#if club}
    <EditClub
      {club}
      clubDetails={clubsData.currentClubDetails || data.publicClubDetails}
      clubChannel={data.clubChannel || undefined}
      {onUpdateSuccess}
    />
  {:else}
    Club not found
  {/if}

  <Footer />
</LimeSection>
