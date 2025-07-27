<script>
  import JuggleElement from './JuggleElement.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} ids
   */

  /** @type {Props} */
  let { ids } = $props();
  let currentIndex = $state(0);
  const videos = $derived(
    ids.map((id) => {
      return { id: id, player: undefined };
    })
  );
  $effect(() => {
    videos[currentIndex].player?.play();
    videos[currentIndex].player?.addEventListener(
      'playbackStatusUpdate',
      nextOnEnded
    );

    const prevIdx = (currentIndex + ids.length - 1) % ids.length;
    const nextIdx = (currentIndex + 1) % ids.length;
    if (prevIdx !== currentIndex) {
      videos[prevIdx].player?.pause();
      videos[prevIdx].player?.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
    if (nextIdx !== currentIndex) {
      videos[nextIdx].player?.pause();
      videos[nextIdx].player?.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
  });

  function prev() {
    currentIndex = (currentIndex + ids.length - 1) % ids.length;
  }

  function next() {
    currentIndex = (currentIndex + 1) % ids.length;
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
</script>

<div class="container">
  <button onclick={prev}>&lt;</button>
  <div class="videos">
    {#each videos as video, index}
      <JuggleElement position={pos(index)}>
        <PeertubeVideoPlayer bind:this={video.player} videoId={video.id} />
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
