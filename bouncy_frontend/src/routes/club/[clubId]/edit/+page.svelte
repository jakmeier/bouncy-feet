<script>
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import EditClub from '$lib/components/club/EditClub.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import SingleActionHeader from '$lib/components/ui/header/SingleActionHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import { deleteClub, getClubsContext } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const clubId = Number.parseInt(page.params.clubId || '0');
  const { clubsData } = getClubsContext();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

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

  async function confirmDelete() {
    const p0 = $t('club.confirm-delete-club-p0');
    const p1 = $t('club.confirm-delete-club-p1');
    const msg = `${p0}${club?.name}${p1}`;

    if (confirm(msg) && club) {
      const ok = await deleteClub(userCtx, club.id);
      if (ok) {
        goto('/profile/#my-clubs');
      }
    }
  }
</script>

<LimeSection>
  <SingleActionHeader
    title={club?.name}
    mainColor
    onAction={confirmDelete}
    button="delete"
  />

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
