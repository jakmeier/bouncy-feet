<script>
  import Juggler from './Juggler.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {string[]} ids
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { ids, autoplay } = $props();
  let juggler = $state();
  let currentIndex = $state(0);
  const videos = $derived(
    ids.map((id) => {
      return {
        id: id,
        /** @type {PeertubeVideoPlayer | undefined} */
        player: undefined,
      };
    })
  );

  /** @type {number[]} */
  let videoHeight = $state([]);
  let buttonHeight = $derived(
    `calc(${videoHeight[currentIndex] / 2}px - 1.5rem)`
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

  /** @param {number} index */
  function onIndexChanged(index) {
    videos[index].player?.forceLoad();
    currentIndex = index;
  }

  /**
   * @param {PeerTubePlayerState} playerState
   */
  function nextOnEnded(playerState) {
    if (playerState.playbackState === 'ended') {
      juggler.next();
    }
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

<Juggler bind:this={juggler} {onIndexChanged} items={videos} {buttonHeight}>
  {#snippet element({ item: video, index })}
    <div bind:clientHeight={videoHeight[index]}>
      <PeertubeVideoPlayer
        bind:this={video.player}
        videoId={video.id}
        delayLoadingMs={delayMs(index)}
      />
    </div>
  {/snippet}
</Juggler>
