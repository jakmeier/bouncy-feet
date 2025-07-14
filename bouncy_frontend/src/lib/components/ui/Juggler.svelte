<script lang="ts">
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
  {#each videos as video, index}
    <JuggleElement position={pos(index)}>
      <PeertubeVideoPlayer bind:this={video.player} videoId={video.id} />
    </JuggleElement>
  {/each}
</div>

<button onclick={next}>Next</button>

<style>
  .container {
    position: relative;
    width: 100vw;
    height: 60vh;
    overflow: hidden;
  }
</style>
