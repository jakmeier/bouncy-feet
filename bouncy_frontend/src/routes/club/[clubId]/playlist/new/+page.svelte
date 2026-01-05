<script>
  import { page } from '$app/state';
  import PlaylistForm from '$lib/components/playlist/PlaylistForm.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import {
    createNewClubPlaylist,
    loadAndSetClubDetails,
  } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();
  const clubId = Number.parseInt(page.params.clubId || '0');

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let name = $state('');
  let description = $state('');

  async function save() {
    const response = await createNewClubPlaylist(
      userCtx,
      clubId,
      name,
      description,
      false
    );
    if (response) {
      loadAndSetClubDetails(clubId, userCtx);
      history.back();
    }
  }
</script>

<LimeSection fillScreen>
  <LogoHeader title={$t('playlist.new-title')} backButton mainColor />

  <PlaylistForm
    bind:name
    bind:description
    userOrChannel={data.clubChannel || undefined}
    onSubmit={save}
  />

  <Footer />
</LimeSection>
