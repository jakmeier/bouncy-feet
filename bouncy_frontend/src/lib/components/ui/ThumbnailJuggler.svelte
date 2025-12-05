<script>
  import { PUBLIC_BF_PEERTUBE_URL } from '$env/static/public';
  import * as api from '$lib/peertube-openapi';
  import FullScreenArea from './FullScreenArea.svelte';
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
  let currentIndex = $state(0);
  let videoId = $state();
  let showPopup = $state(false);

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
</script>

<PopupWithRunes bind:isOpen={showPopup} {onClose}>
  {#if videoId}
    <FullScreenArea>
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
    </FullScreenArea>
  {/if}
</PopupWithRunes>

<Juggler bind:this={juggler} {onIndexChanged} items={videos}>
  {#snippet element({ item: video })}
    <UnstyledButton onClick={() => playVideo(video)}>
      <img
        src="{PUBLIC_BF_PEERTUBE_URL}{video.thumbnailPath}"
        alt="thumbnail"
      />
    </UnstyledButton>
  {/snippet}
</Juggler>

<style>
  .popup {
    background-color: var(--theme-neutral-black);
    height: 100%;
    width: 100%;
  }

  .close {
    display: grid;
    place-items: center;
    height: 3rem;
    width: 100%;
    position: absolute;
  }

  .video {
    display: grid;
    place-items: center;
    height: 100%;
  }
</style>
