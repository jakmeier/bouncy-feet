<script>
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import { VIDEO_PRIVACY } from '$lib/peertube';
  import ThumbnailFeed from '../ThumbnailFeed.svelte';
  import VideoUpload from '../ui/video/VideoUpload.svelte';
  import * as api from '$lib/peertube-openapi';

  /**
   * @typedef {Object} Props
   * @property {api.VideoPlaylist} playlist
   * @property {number} clubId
   */
  /** @type {Props} */
  let { playlist, clubId } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let videoUpload = $state();
  let videoFeed = $state();

  const uploadPrivacy = VIDEO_PRIVACY.UNLISTED;

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
        playlist_id: playlist.id,
      })
      .then(() => videoFeed?.refreshVideos());
  }
</script>

<!-- <p>{$t('club.upload-video-description')}</p> -->

<button onclick={() => videoUpload.open()}>
  {$t('club.upload-video-button')}
</button>

{#if playlist.shortUUID}
  <ThumbnailFeed playlistUuid={playlist.shortUUID} bind:this={videoFeed}
  ></ThumbnailFeed>
{/if}

<div class="hidden">
  <VideoUpload bind:this={videoUpload} {onVideoUploaded} privacy={uploadPrivacy}
  ></VideoUpload>
</div>

<style>
  .hidden {
    display: None;
  }

  button {
    margin-bottom: 2rem;
  }
</style>
