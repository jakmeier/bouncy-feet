<script>
  import JuggleElement from './JuggleElement.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} ids
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { ids, autoplay } = $props();
  let currentIndex = $state(0);
  const videos = $derived(
    // reverse for z-ordering
    [...ids].reverse().map((id) => {
      return {
        id: id,
        /** @type {PeertubeVideoPlayer | undefined} */
        player: undefined,
      };
    })
  );
  $effect(() => {
    const currentVideo = videos[currentIndex];
    if (autoplay && currentVideo.player) {
      currentVideo.player.play();
      currentVideo.player.addEventListener('playbackStatusUpdate', nextOnEnded);
    }

    const prevIdx = (currentIndex + ids.length - 1) % ids.length;
    const nextIdx = (currentIndex + 1) % ids.length;
    if (prevIdx !== currentIndex && videos[prevIdx].player) {
      videos[prevIdx].player.pause();
      videos[prevIdx].player.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
    if (nextIdx !== currentIndex && videos[nextIdx].player) {
      videos[nextIdx].player.pause();
      videos[nextIdx].player.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
  });

  function prev() {
    currentIndex = (currentIndex + ids.length - 1) % ids.length;
    videos[currentIndex].player?.forceLoad();
  }

  function next() {
    currentIndex = (currentIndex + 1) % ids.length;
    videos[currentIndex].player?.forceLoad();
  }

  /**
   * @param {PeerTubePlayerState} playerState
   */
  function nextOnEnded(playerState) {
    if (playerState.playbackState === 'ended') {
      next();
    }
  }

  /**
   * @param {number} index
   * @returns {"left"|"right"|"center"}
   */
  function pos(index) {
    if (index < currentIndex) {
      return 'left';
    }
    if (index > currentIndex) {
      return 'right';
    }
    return 'center';
  }

  /**
   * Loading all at once may hit rate limits, so delay loading
   * @param {number} index
   * @return {number} ms
   */
  function delayMs(index) {
    const delta = Math.abs(index - currentIndex);
    if (delta < 3) {
      // Load the first few fairly quickly
      return delta * 500;
    } else {
      // delay the rest for much longer
      return (delta - 2) * 5000;
    }
  }
</script>

<div class="container">
  <button onclick={prev}>&lt;</button>
  <div class="videos">
    {#each videos as video, reverseIndex}
      {@const index = videos.length - 1 - reverseIndex}
      <JuggleElement position={pos(index)}>
        <PeertubeVideoPlayer
          bind:this={video.player}
          videoId={video.id}
          delayLoadingMs={delayMs(index)}
        />
      </JuggleElement>
    {/each}
  </div>
  <button onclick={next}>&gt;</button>
</div>

<style>
  .container {
    position: relative;
    width: 100%;
    height: 70vh;
  }

  .container button {
    position: absolute;
    z-index: 1;
    top: 50%;
    width: 3rem;
    height: 3rem;
    padding: 0;
    margin: 0;
    border-radius: 50%;
    min-width: initial;
    max-width: initial;
  }

  .container button:first-child {
    left: -1.5rem;
  }
  .container button:last-child {
    right: -1.5rem;
  }

  .videos {
    overflow: hidden;
  }
</style>
