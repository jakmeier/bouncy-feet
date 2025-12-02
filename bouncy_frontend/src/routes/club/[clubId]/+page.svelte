<script>
  import { page } from '$app/state';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import PopupWithRunes from '$lib/components/ui/PopupWithRunes.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';
  import UserList from '$lib/components/UserList.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import { getClubsContext } from '$lib/stores/Clubs.svelte';

  const clubId = Number.parseInt(page.params.clubId || '0');

  /** @type {UserContextData} */
  const userCtx = getUserContext();
  const { clubsData } = getClubsContext();

  /** @type {Club} */
  const club = $derived(clubsData.mine.find((c) => c.id === clubId));
  const playlistId = $derived(club.private_playlist?.id);

  let showPopup = $state(false);
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

    showPopup = false;
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
</script>

<h2>{club.name}</h2>
<p>{club.description}</p>

<!-- TODO: Clean up design etc -->
<button onclick={() => (showPopup = true)}>{$t('club.add-user-button')}</button>

<!-- TODO: clean up, maybe put in components etc -->
<LoginRequiredContent reason={$t('profile.upload.requires-login-description')}>
  <p>Add club video</p>
  <VideoUpload {onVideoUploaded}></VideoUpload>
</LoginRequiredContent>

<PopupWithRunes bind:isOpen={showPopup}>
  <div class="popup">
    {#if message}
      <div>{message}</div>
    {:else}
      <div>{$t('club.select-user-title')}</div>
      <UserList onSelect={onSelectUser}></UserList>
    {/if}
  </div>
</PopupWithRunes>

<style>
  .popup {
    background-color: var(--theme-main);
    color: var(--theme-neutral-dark);
    padding: 1rem;
    border-radius: 1rem;
  }
</style>
