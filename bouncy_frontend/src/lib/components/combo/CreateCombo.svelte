<script>
  import * as api from '$lib/peertube-openapi';
  import { t } from '$lib/i18n';
  import PeertubeVideoPlayer from '../ui/video/PeertubeVideoPlayer.svelte';
  import VideoUpload from '../ui/video/VideoUpload.svelte';
  import { VIDEO_PRIVACY } from '$lib/peertube';
  import VideoLoader from '../ui/video/VideoLoader.svelte';
  import ComboForm from './ComboForm.svelte';

  /**
   * @typedef {Object} Props
   * @prop {FullUser} fullUser
   * @prop {ApiUser} apiUser
   *
   */

  /** @type {Props}*/
  let { fullUser, apiUser } = $props();

  /** @type {ComboInfo | CreateComboRequest}*/
  let combo = $state({
    is_private: true,
  });
  /** @type {api.VideoPrivacySet}*/
  let privacy = $derived(
    combo.is_private ? VIDEO_PRIVACY.UNLISTED : VIDEO_PRIVACY.PUBLIC
  );
  /** @type {api.Video | undefined}*/
  let video = $state();
  let dirty = $state(false);

  async function saveAndLeave() {
    await saveCombo();
    history.back();
  }

  async function saveCombo() {
    let response;
    if (combo.hasOwnProperty('id')) {
      response = await apiUser.authenticatedPost('/combos/update', combo);
    } else {
      response = await apiUser.authenticatedPost('/combos/new', combo);
    }
    if (response?.ok) {
      /** @type {ComboInfo}*/
      const newCombo = await response.json();
      combo = newCombo;
      dirty = false;
    }
  }

  /**
   * @param {import('$lib/peertube-openapi').VideoUploadResponse} video
   */
  async function onVideoUploaded(video) {
    if (video.video?.shortUUID) {
      combo.video_short_uuid = video.video.shortUUID;
      saveCombo();
    }
  }
</script>

{#if !combo.video_short_uuid}
  <p>{$t('profile.combo.create-description')}</p>
  <p>{$t('profile.combo.add-video-description')}</p>
  <VideoUpload {fullUser} {onVideoUploaded} {privacy}></VideoUpload>
{:else if !video}
  <VideoLoader videoId={combo.video_short_uuid} onLoaded={(v) => (video = v)} />
{:else}
  <div class="video">
    <PeertubeVideoPlayer
      videoId={combo.video_short_uuid}
      aspectRatio={video.aspectRatio || 1}
      timeline="external"
    />
  </div>

  <div class="form">
    <ComboForm bind:details={combo} bind:dirty></ComboForm>
  </div>

  <button onclick={saveAndLeave} disabled={!dirty}
    >{$t('profile.combo.save-button')}</button
  >
{/if}

<style>
  .form {
    margin: 1rem 0;
  }
</style>
