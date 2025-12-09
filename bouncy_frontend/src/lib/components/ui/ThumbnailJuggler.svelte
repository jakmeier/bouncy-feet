<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import * as api from '$lib/peertube-openapi';
  import Juggler from './Juggler.svelte';
  import PopupWithRunes from './PopupWithRunes.svelte';
  import Symbol from './Symbol.svelte';
  import UnstyledButton from './UnstyledButton.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {api.Video[]} videos
   */

  /** @type {Props} */
  let { videos } = $props();
  let juggler = $state();
  /** @type {number[]} */
  let imageHeight = $state([]);
  let currentIndex = $state(0);
  let videoId = $state();
  let showPopup = $state(false);

  let buttonHeight = $derived(
    `calc(${imageHeight[currentIndex] / 2}px - 1.5rem)`
  );

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
          <div class="name">
            {video.name}
          </div>
        {/if}
      </div>
    </UnstyledButton>
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
    height: calc(3 * var(--font-normal));
    padding: 0.5rem;
  }

  .name:hover {
    background-color: var(--theme-neutral-light);
    border-radius: 0.5rem;
    overflow: visible;
    height: auto;
  }
</style>
