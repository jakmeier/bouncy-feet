<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import { t } from '$lib/i18n';
  import { updateVideo, VIDEO_PRIVACY } from '$lib/peertube';
  import * as api from '$lib/peertube-openapi';
  import { privacySymbol, privacyText } from '$lib/peertube_utils';
  import Juggler from './Juggler.svelte';
  import PopupWithRunes from './PopupWithRunes.svelte';
  import PrivacySelector from './PrivacySelector.svelte';
  import StringEditor from './StringEditor.svelte';
  import Symbol from './Symbol.svelte';
  import UnstyledButton from './UnstyledButton.svelte';
  import FullScreenVideo from './video/FullScreenVideo.svelte';

  /**
   * @typedef {Object} Props
   * @property {api.Video[]} videos
   * @property {boolean} [userExtraInfo]
   * @property {boolean} [clubExtraInfo]
   * @property {(video: api.Video, index: number)=>void} [onDelete]
   */

  /** @type {Props} */
  let {
    videos = $bindable(),
    userExtraInfo = false,
    clubExtraInfo = false,
    onDelete,
  } = $props();
  let juggler = $state();
  /** @type {number[]} */
  let imageHeight = $state([]);
  let currentIndex = $state(0);
  let videoId = $state();
  let aspectRatio = $state();
  let showVideoPopup = $state(false);
  let showPrivacyPopUp = $state(false);
  let showEditPopUp = $state(false);
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
  let reactiveName = $derived(videos[currentIndex].name);

  /** @param {number} index */
  function onIndexChanged(index) {
    currentIndex = index;
  }

  /** @param {api.Video} video */
  function playVideo(video) {
    videoId = video.uuid;
    aspectRatio = video.aspectRatio;
    showVideoPopup = true;
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

  function openNameEdit() {
    showEditPopUp = true;
  }

  /**
   * @param {string} value
   */
  async function onNameChanged(value) {
    if (value.length > 0) {
      const video = videos[currentIndex];
      const id = video.id || video.uuid || video.shortUUID;
      if (id) {
        // update PeerTube through api
        const updated = await updateVideo(id, { name: value });
        // update UI
        if (updated) {
          video.name = value;
          // write to $derived to trigger the update that gets missed otherwise
          reactiveName = video.name;
        }
      }
    }
    showEditPopUp = false;
  }
</script>

{#if videoId}
  <FullScreenVideo bind:isOpen={showVideoPopup} {videoId} {aspectRatio} {onClose} />
{/if}

<PopupWithRunes bind:isOpen={showPrivacyPopUp}>
  <PrivacySelector selected={selectedPrivacy} onSelected={updatePrivacy}
  ></PrivacySelector>
</PopupWithRunes>

<PopupWithRunes bind:isOpen={showEditPopUp}>
  <div class="edit-pop-up">
    <StringEditor
      initValue={reactiveName}
      label="video.name"
      onSelected={onNameChanged}
    ></StringEditor>
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
        {#if index === currentIndex && !userExtraInfo}
          <p class="name">
            {video.name}
          </p>
        {/if}
      </div>
    </UnstyledButton>
    {#if onDelete && index === currentIndex}
      <div class="delete-button">
        <UnstyledButton onClick={() => onDelete(video, index)}>
          <Symbol size={48}>delete</Symbol>
        </UnstyledButton>
      </div>
    {/if}
    {#if clubExtraInfo && index === currentIndex}
      <div class="extra-info">
        <Symbol size={32}>lock</Symbol>
        <p>{video.account.displayName}</p>
      </div>
    {/if}

    {#if userExtraInfo && index === currentIndex}
      <div class="extra-info">
        <UnstyledButton onClick={() => openNameEdit()}>
          <Symbol size={32}>edit</Symbol>
        </UnstyledButton>
        <UnstyledButton onClick={() => openNameEdit()}>
          <p>
            {reactiveName}
          </p>
        </UnstyledButton>

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
  .edit-pop-up {
    color: var(--theme-main);
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
    max-height: calc(2.5 * var(--font-normal));
    padding: 0.5rem;
  }

  .name:hover {
    background-color: var(--theme-neutral-light);
    color: var(--theme-neutral-dark);
    border-radius: 0.5rem;
    height: auto;
    min-height: calc(2.5 * var(--font-normal));
  }

  p {
    margin: 0;
    overflow: hidden;
    overflow-wrap: break-word;
  }

  .extra-info {
    display: grid;
    grid-template-columns: 4rem auto;
    align-items: center;
    margin-top: 1rem;
    gap: 1rem 0;
  }

  .delete-button {
    text-align: center;
    color: var(--theme-red);
    margin-top: -1rem;
  }
</style>
