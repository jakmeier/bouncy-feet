<script>
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';
  import ThumbnailFeed from '$lib/components/ThumbnailFeed.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import PopupWithRunes from '$lib/components/ui/PopupWithRunes.svelte';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Playlist from '$lib/components/ui/video/Playlist.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';
  import UserList from '$lib/components/user/UserList.svelte';
  import { getClubsContext, getUserContext } from '$lib/stores/context';
  import { t } from '$lib/i18n';
  import { VIDEO_PRIVACY } from '$lib/peertube';

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

  const clubDetails = $derived(
    clubsData.currentClubDetails || data.publicClubDetails
  );
  const isClubMember = $derived.by(
    () => clubsData.mine.findIndex((c) => c.id === clubId) !== -1
  );
  // TODO: isAdmin check
  // const isAdmin = $derived(clubDetails.admins.findIndex((u) => u.id === myId) !== -1);
  const isAdmin = true;

  let showUsersPopup = $state(false);
  let showAddMorePopup = $state(false);
  let showAddVideoPopup = $state(false);
  let message = $state('');
  let mainFeed = $state();
  /** @type {Playlist[]} */
  let privatePlaylists = $state([]);
  // svelte-ignore state_referenced_locally
  let uploadToPlaylist = $state(clubDetails.main_playlist?.id);
  let uploadPrivacy = $state(VIDEO_PRIVACY.PUBLIC);
  /** @type {{refreshVideos: ()=>void} | undefined} */
  let uploadToFeed = $state();

  /**
   * @param {PublicUserResponse} user
   * @param {ApiUser} apiUser
   */
  async function onSelectUser(user, apiUser) {
    const p0 = $t('club.confirm-add-user-p0');
    const p1 = $t('club.confirm-add-user-p1');
    const p2 = $t('club.confirm-add-user-p2');
    const msg = `${p0}${user.display_name}${p1}${club?.name}${p2}`;
    if (confirm(msg)) {
      /** @type {boolean} */
      let ok = false;
      try {
        const result = await apiUser.authenticatedPost('/clubs/add_member', {
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
   * @param {ApiUser} apiUser
   */
  function onVideoUploaded(video, apiUser) {
    if (!video.video) {
      console.error('Got no video to add');
      return;
    }
    let videoId = video.video.id;
    const targetFeed = uploadToFeed;
    apiUser
      .authenticatedPost('/clubs/video/add', {
        video_id: videoId,
        club_id: clubId,
        playlist_id: uploadToPlaylist,
      })
      .then(() => targetFeed?.refreshVideos());
    showAddVideoPopup = false;
  }

  function onAddMore() {
    showAddMorePopup = true;
  }

  function openEdit() {
    goto('./edit');
  }

  /**
   * @param {number | undefined} playlistId
   * @param {import('$lib/peertube-openapi').VideoPrivacySet} privacy
   * @param {{refreshVideos: ()=>void} | undefined} feed
   */
  function openVideoUpload(playlistId, privacy, feed) {
    showAddMorePopup = false;
    showAddVideoPopup = true;
    uploadToPlaylist = playlistId;
    uploadPrivacy = privacy;
    uploadToFeed = feed;
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
      <ActorAvatar actor={data.clubChannel || undefined} />

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

    {#if clubDetails.main_playlist}
      <h2>{$t('club.public-videos-title')}</h2>
      <ThumbnailFeed
        bind:this={mainFeed}
        playlistUuid={clubDetails.main_playlist.short_uuid}
      ></ThumbnailFeed>
    {/if}

    {#each clubDetails.public_playlists as playlist}
      {#if playlist.id != clubDetails.main_playlist?.id}
        <Playlist playlistInfo={playlist} editable={isClubMember} />
      {/if}
    {/each}
  {/if}
</LimeSection>

<NightSection>
  {#if clubDetails.private}
    <h2>{$t('club.private-videos-title')}</h2>
    {#each clubDetails.private.private_playlists as playlist, i}
      <div class="playlist">
        <Playlist
          bind:this={privatePlaylists[i]}
          playlistInfo={playlist}
          editable={isClubMember}
        />
      </div>
    {/each}
  {/if}
</NightSection>

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

<PopupWithRunes bind:isOpen={showUsersPopup}>
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    {#snippet children({ apiUser })}
      <div class="popup">
        {#if message}
          <div>{message}</div>
        {:else}
          <div>{$t('club.select-user-title')}</div>
          <UserList onSelect={(u) => onSelectUser(u, apiUser)} {apiUser}
          ></UserList>
        {/if}
      </div>
    {/snippet}
  </LoginRequiredContent>
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showAddMorePopup}>
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    <div class="add-more">
      <Button
        symbol="boy"
        text="club.add-user-button"
        class="full-width"
        on:click={() => {
          showAddMorePopup = false;
          showUsersPopup = true;
        }}
      />
      <Button
        symbol="upload"
        text={'club.upload-video-button'}
        class="full-width"
        on:click={() =>
          openVideoUpload(
            clubDetails.main_playlist?.id,
            VIDEO_PRIVACY.PUBLIC,
            mainFeed
          )}
      />
      <Button
        symbol="perm_media"
        text={'club.add-playlist-button'}
        class="full-width"
        on:click={() => goto('./playlist/new')}
      />
    </div>
  </LoginRequiredContent>
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showAddVideoPopup}>
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    {#snippet children({ fullUser, apiUser })}
      <p>{$t('club.upload-video-description')}</p>
      <VideoUpload
        onVideoUploaded={(video) => onVideoUploaded(video, apiUser)}
        privacy={uploadPrivacy}
        {fullUser}
      ></VideoUpload>
    {/snippet}
  </LoginRequiredContent>
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

  .add-more {
    min-width: 200px;
    display: grid;
    width: 100%;
    gap: 1rem;
  }

  .playlist {
    background-color: var(--theme-neutral-almost-black);
    color: var(--theme-neutral-white);
    border-radius: 1rem;
    padding: 0 1rem;
    margin: 1rem 0;
  }
</style>
