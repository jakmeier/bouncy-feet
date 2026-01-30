<script>
  import { page } from '$app/state';
  import PlaylistForm from '$lib/components/playlist/PlaylistForm.svelte';
  import PlaylistVideosForm from '$lib/components/playlist/PlaylistVideosForm.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import ClubInfoHeader from '$lib/components/ui/header/ClubInfoHeader.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { t } from '$lib/i18n';
  import {
    loadAndSetClubDetails,
    updateClubPlaylist,
  } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();
  const clubId = Number.parseInt(page.params.clubId || '0');

  let name = $state(data.playlist.displayName);
  let description = $state(data.playlist.description);

  /** @param {ApiUser} apiUser */
  async function save(apiUser) {
    const response = await updateClubPlaylist(
      apiUser,
      clubId,
      data.playlist.id,
      name,
      description,
      false
    );
    if (response) {
      loadAndSetClubDetails(clubId, apiUser);
      history.back();
    }
  }
</script>

<ScrollToTop />

<LoginRequiredContent reason="">
  {#snippet children({ apiUser, fullUser })}
    <LimeSection arrow arrowText="Videos">
      {#if data.clubChannel}
        <ClubInfoHeader
          title={$t('playlist.edit-title')}
          userOrChannel={data.clubChannel}
          mainColor
        />
      {:else}
        <BackHeader mainColor title={$t('playlist.edit-title')} />
      {/if}

      <div class="form">
        <PlaylistForm
          bind:name
          bind:description
          onSubmit={() => save(apiUser)}
        />
      </div>
    </LimeSection>

    <DarkSection>
      <h1>{$t('playlist.edit-videos-title')}</h1>

      <PlaylistVideosForm {clubId} playlist={data.playlist} {apiUser} {fullUser}
      ></PlaylistVideosForm>

      <Footer white />
    </DarkSection>
  {/snippet}
</LoginRequiredContent>

<style>
  .form {
    margin-bottom: 2rem;
  }
</style>
