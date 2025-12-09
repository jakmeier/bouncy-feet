<script>
  import { page } from '$app/state';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import ThumbnailFeed from '$lib/components/ThumbnailFeed.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import PopupWithRunes from '$lib/components/ui/PopupWithRunes.svelte';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';
  import UserList from '$lib/components/UserList.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import { VIDEO_PRIVACY } from '$lib/peertube';
  import { getClubsContext } from '$lib/stores/Clubs.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const clubId = Number.parseInt(page.params.clubId || '0');

  /** @type {UserContextData} */
  const userCtx = getUserContext();
  const { clubsData } = getClubsContext();

  // May be undefined while clubs are still loading.
  /** @type {Club | undefined} */
  const club = $derived.by(
    () =>
      clubsData.mine.find((c) => c.id === clubId) ||
      clubsData.public.find((c) => c.id === clubId)
  );

  let showUsersPopup = $state(false);
  let showAddMorePopup = $state(false);
  let showAddVideoPopup = $state(false);
  let message = $state('');

  /**
   * @param {PublicUserResponse} user
   */
  async function onSelectUser(user) {
    const p0 = $t('club.confirm-add-user-p0');
    const p1 = $t('club.confirm-add-user-p1');
    const p2 = $t('club.confirm-add-user-p2');
    const msg = `${p0}${user.display_name}${p1}${club?.name}${p2}`;
    if (confirm(msg)) {
      /** @type {boolean} */
      let ok = false;
      try {
        const result = await userCtx.authenticatedPost('/clubs/add_member', {
          user_id: Number(user.id),
          club_id: club?.id,
        });
        ok = result?.ok || false;
      } catch (err) {
        console.error('failed adding user', err);
      }

      if (ok) {
        message = $t('club.add-user-success');
      } else {
        message = $t('club.add-user-failed');
      }
      await new Promise((r) => setTimeout(r, 1200));
    }

    showUsersPopup = false;
  }

  /**
   * @param {import('$lib/peertube-openapi').VideoUploadResponse} video
   */
  function onVideoUploaded(video) {
    if (!video.video) {
      console.error('Got no video to add');
      return;
    }
    let videoId = video.video.id;
    userCtx.authenticatedPost('/clubs/video/add', {
      video_id: videoId,
      club_id: clubId,
      private: true,
    });
  }

  function onAddMore() {
    showAddMorePopup = true;
  }
</script>

<LimeSection>
  <LogoHeader title={club?.name} backButton onAction={onAddMore} mainColor
  ></LogoHeader>

  {#if !club}
    <div class="loading">
      <Symbol size={100} styleClass="rotating">refresh</Symbol>
    </div>
  {:else}
    <p>{club.description}</p>

    {#if club.private_playlist}
      <LoginRequiredContent reason={$t('club.requires-login-description')}>
        <h2>Private Club Videos</h2>
        <ThumbnailFeed playlistId={club.private_playlist.short_uuid}
        ></ThumbnailFeed>
      </LoginRequiredContent>
    {/if}

    <h2>Public Club Videos</h2>
    <ThumbnailFeed playlistId={club.public_playlist.short_uuid}></ThumbnailFeed>

    <PopupWithRunes bind:isOpen={showUsersPopup}>
      <div class="popup">
        {#if message}
          <div>{message}</div>
        {:else}
          <div>{$t('club.select-user-title')}</div>
          <UserList onSelect={onSelectUser}></UserList>
        {/if}
      </div>
    </PopupWithRunes>
  {/if}
</LimeSection>

<LightSection>
  <h2>{$t('club.admins-title')}</h2>
  <ul>
    {#each data.admins as user}
      <li>{user.display_name}</li>
    {/each}
  </ul>
  <h2>{$t('club.members-title')}</h2>
  <ul>
    {#each data.members as user}
      <li>{user.display_name}</li>
    {/each}
  </ul>
  <Footer />
</LightSection>

<PopupWithRunes bind:isOpen={showAddMorePopup}>
  {#if userCtx.isLoggedInToApi()}
    <Button
      symbol="boy"
      text="club.add-user-button"
      on:click={() => {
        showAddMorePopup = false;
        showUsersPopup = true;
      }}
    />
    <Button
      symbol="upload"
      text={'club.upload-video-button'}
      on:click={() => {
        showAddMorePopup = false;
        showAddVideoPopup = true;
      }}
    />
  {/if}
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showAddVideoPopup}>
  {#if userCtx.isLoggedInToApi()}
    <p>Add club video</p>
    <VideoUpload {onVideoUploaded} privacy={VIDEO_PRIVACY.UNLISTED}
    ></VideoUpload>
  {/if}
</PopupWithRunes>

<style>
  .popup {
    background-color: var(--theme-main);
    color: var(--theme-neutral-dark);
    padding: 1rem;
    border-radius: 1rem;
  }

  .loading {
    text-align: center;
    margin: 2rem 0;
  }
</style>
