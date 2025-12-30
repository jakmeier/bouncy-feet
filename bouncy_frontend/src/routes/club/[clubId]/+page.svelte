<script>
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';
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

  const clubDetails = $derived(
    clubsData.currentClubDetails || data.publicClubDetails
  );
  // TODO: isAdmin check
  // const isAdmin = $derived(clubDetails.admins.findIndex((u) => u.id === myId) !== -1);
  const isAdmin = true;

  let showUsersPopup = $state(false);
  let showAddMorePopup = $state(false);
  let showAddVideoPopup = $state(false);
  let message = $state('');
  let mainFeed = $state();

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
    userCtx
      .authenticatedPost('/clubs/video/add', {
        video_id: videoId,
        club_id: clubId,
        private: false,
      })
      .then(() => mainFeed.refreshVideos());
    showAddVideoPopup = false;
  }

  function onAddMore() {
    showAddMorePopup = true;
  }

  function openEdit() {
    goto('./edit');
  }
</script>

<ScrollToTop />

<LimeSection>
  <LogoHeader
    title={club?.name}
    backButton
    onAction={onAddMore}
    mainColor
    onSecondAction={openEdit}
    secondButton="edit"
  ></LogoHeader>

  {#if !club}
    <div class="loading">
      <Symbol size={100} styleClass="rotating">refresh</Symbol>
    </div>
  {:else}
    <div class="club-summary">
      <ActorAvatar actor={data.clubChannel} />

      <div class="club-description">
        <div>{club.description}</div>
      </div>
    </div>
    {#if clubDetails.web_link}
      <a class="link" href={clubDetails.web_link}>{clubDetails.web_link}</a>
    {/if}

    <p>
      {clubDetails.num_members + clubDetails.admins.length}
      {$t('club.members-title')}
    </p>

    <h2>{$t('club.public-videos-title')}</h2>
    <ThumbnailFeed
      bind:this={mainFeed}
      playlistId={clubDetails.main_playlist.id}
    ></ThumbnailFeed>

    {#each clubDetails.public_playlists as playlist}
      {#if playlist.id != clubDetails.main_playlist.id}
        <!-- TODO: playlist title -->
        <h2>Playlist {playlist.id}</h2>
        <ThumbnailFeed playlistId={playlist.id}></ThumbnailFeed>
      {/if}
    {/each}

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
    {#each clubDetails.admins as user}
      <li>{user.display_name}</li>
    {/each}
  </ul>
  {#if clubDetails.private}
    <h2>{$t('club.members-title')}</h2>
    <ul>
      {#each clubDetails.private.members as user}
        <li>{user.display_name}</li>
      {/each}
    </ul>
  {/if}
  <Footer />
</LightSection>

<PopupWithRunes bind:isOpen={showAddMorePopup}>
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
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
  </LoginRequiredContent>
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showAddVideoPopup}>
  {#if userCtx.isLoggedInToApi()}
    <p>{$t('club.upload-video-description')}</p>
    <VideoUpload {onVideoUploaded} privacy={VIDEO_PRIVACY.PUBLIC}></VideoUpload>
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

  .link {
    text-decoration: underline;
  }
  .link:hover {
    color: var(--theme-accent-dark);
  }

  .club-summary {
    display: grid;
    grid-template-columns: 1fr 3fr;
    gap: 1rem;
    margin-bottom: 1rem;
  }
</style>
