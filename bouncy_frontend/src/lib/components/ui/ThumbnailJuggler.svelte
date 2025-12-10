<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { t } from '$lib/i18n';
  import { updateVideo, VIDEO_PRIVACY } from '$lib/peertube';
  import * as api from '$lib/peertube-openapi';
  import { privacySymbol, privacyText } from '$lib/peertube_utils';
  import Juggler from './Juggler.svelte';
  import PopupWithRunes from './PopupWithRunes.svelte';
  import PrivacySelector from './PrivacySelector.svelte';
  import Symbol from './Symbol.svelte';
  import UnstyledButton from './UnstyledButton.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {api.Video[]} videos
   * @property {boolean} [extraInfo]
   */

  /** @type {Props} */
  let { videos = $bindable(), extraInfo = false } = $props();
  let juggler = $state();
  /** @type {number[]} */
  let imageHeight = $state([]);
  let currentIndex = $state(0);
  let videoId = $state();
  let showPopup = $state(false);
  let showPrivacyPopUp = $state(false);
  /** @param {api.VideoPrivacySet} id */
  let selectedPrivacy = $state(VIDEO_PRIVACY.PRIVATE);

  let buttonHeight = $derived(
    `calc(${imageHeight[currentIndex] / 2}px - 1.5rem)`
  );
  let currentPrivacy = $derived(
    videos[currentIndex].privacy?.id || VIDEO_PRIVACY.PUBLIC
  );
  const reactivePrivacySymbol = $derived(privacySymbol(currentPrivacy));
  const reactivePrivacyText = $derived(privacyText(currentPrivacy));

  /** @param {number} index */
  function onIndexChanged(index) {
    currentIndex = index;
  }

  /** @param {api.Video} video */
  function playVideo(video) {
    videoId = video.uuid;
    showPopup = true;
  }

  function onClose() {
    videoId = undefined;
  }

  function formatDuration(seconds) {
    const full = new Date(1000 * seconds).toISOString().substring(11, 19);
    if (full.startsWith('00:')) {
      return full.substring(3);
    } else {
      return full;
    }
  }

  /** @param {api.Video} video */
  function openPrivacySettings(video) {
    selectedPrivacy = video.privacy?.id || VIDEO_PRIVACY.PUBLIC;
    showPrivacyPopUp = true;
  }

  /**
   * @param {api.VideoPrivacySet} privacy
   */
  async function updatePrivacy(privacy) {
    const video = videos[currentIndex];
    const id = video.id || video.uuid || video.shortUUID;
    if (id) {
      // update PeerTube through api
      const updated = await updateVideo(id, { privacy });
      // update UI
      if (updated && video.privacy) {
        video.privacy.id = privacy;
        // write to $derived to trigger the update that gets missed otherwise
        currentPrivacy =
          videos[currentIndex].privacy?.id || VIDEO_PRIVACY.PUBLIC;
      }
    }
    showPrivacyPopUp = false;
  }
</script>

<PopupWithRunes bind:isOpen={showPopup} {onClose}>
  {#if videoId}
    <div class="popup">
      <div class="close">
        <UnstyledButton onClick={() => (showPopup = false)}>
          <Symbol styleClass="white" size={32}>close</Symbol>
        </UnstyledButton>
      </div>
      <div class="video">
        <PeertubeVideoPlayer {videoId}></PeertubeVideoPlayer>
      </div>
    </div>
  {/if}
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showPrivacyPopUp}>
  <div class="privacy-pop-up">
    <PrivacySelector selected={selectedPrivacy} onSelected={updatePrivacy}
    ></PrivacySelector>
  </div>
</PopupWithRunes>

<Juggler bind:this={juggler} {onIndexChanged} items={videos} {buttonHeight}>
  {#snippet element({ item: video, index })}
    <UnstyledButton onClick={() => playVideo(video)}>
      <div class="preview">
        <div class="thumbnail">
          <img
            src="{PUBLIC_BF_PEERTUBE_URL}{video.thumbnailPath}"
            alt="thumbnail"
            bind:clientHeight={imageHeight[index]}
          />
          <div class="duration">
            {formatDuration(video.duration)}
          </div>
        </div>
        {#if index === currentIndex}
          <p class="name">
            {video.name}
          </p>
        {/if}
      </div>
    </UnstyledButton>
    {#if extraInfo && index === currentIndex}
      <div class="extra-info">
        <UnstyledButton onClick={() => openPrivacySettings(video)}>
          <Symbol size={32}>{reactivePrivacySymbol}</Symbol>
        </UnstyledButton>
        <UnstyledButton onClick={() => openPrivacySettings(video)}>
          <p>
            {$t(reactivePrivacyText)}
          </p>
        </UnstyledButton>
      </div>
    {/if}
  {/snippet}
</Juggler>

<style>
  .popup {
    background-color: var(--theme-neutral-black);
    width: 100vw;
    height: 100vh;
  }

  .close {
    display: grid;
    place-items: center;
    height: 3rem;
    width: 100%;
    position: absolute;
    z-index: 11; /* above video overlay */
  }

  .video {
    display: grid;
    place-items: center;
    height: 100%;
    max-height: calc(90vh - 3rem);
    width: auto;
  }

  .preview {
    display: inline-block;
    width: min(280px, calc(100vw - 3rem)); /* PeerTube thumbnail width */
  }

  .thumbnail {
    position: relative;
  }

  .thumbnail img {
    display: block; /* Remove extra space from display: inline; */
    border-radius: 0.5rem;
    width: min(280px, calc(100vw - 3rem)); /* PeerTube thumbnail width */
  }

  .duration {
    position: absolute;
    bottom: 0;
    right: 0;
    color: var(--theme-neutral-light);
    background-color: var(--theme-neutral-dark-transparent);
    border-radius: 0.5rem;
    padding: 0.25rem;
    margin: 0.25rem;
    line-height: 1rem;
  }

  .name {
    word-wrap: break-word;
    overflow: hidden;
    text-overflow: ellipsis;
    height: calc(2.5 * var(--font-normal));
    padding: 0.5rem;
  }

  .name:hover {
    background-color: var(--theme-neutral-light);
    border-radius: 0.5rem;
    height: auto;
    min-height: calc(2.5 * var(--font-normal));
  }

  p {
    margin: 0;
  }

  .extra-info {
    display: grid;
    grid-template-columns: 4rem auto;
    align-items: center;
  }
</style>
