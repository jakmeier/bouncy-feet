<script>
  import { page } from '$app/state';
  import PlaylistForm from '$lib/components/playlist/PlaylistForm.svelte';
  import PlaylistVideosForm from '$lib/components/playlist/PlaylistVideosForm.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import ClubInfoHeader from '$lib/components/ui/header/ClubInfoHeader.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import {
    loadAndSetClubDetails,
    updateClubPlaylist,
  } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();
  const clubId = Number.parseInt(page.params.clubId || '0');

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let name = $state(data.playlist.displayName);
  let description = $state(data.playlist.description);

  async function save() {
    const response = await updateClubPlaylist(
      userCtx,
      clubId,
      data.playlist.id,
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

<ScrollToTop />

<LoginRequiredContent reason="">
  <LimeSection arrow arrowText="Videos">
    <ClubInfoHeader
      title={$t('playlist.edit-title')}
      userOrChannel={data.clubChannel}
      mainColor
    />

    <div class="form">
      <PlaylistForm bind:name bind:description onSubmit={save} />
    </div>
  </LimeSection>

  <DarkSection>
    <h1>{$t('playlist.edit-videos-title')}</h1>

    <PlaylistVideosForm {clubId} playlist={data.playlist}></PlaylistVideosForm>

    <Footer white />
  </DarkSection>
</LoginRequiredContent>

<style>
  .form {
    margin-bottom: 2rem;
  }
</style>
