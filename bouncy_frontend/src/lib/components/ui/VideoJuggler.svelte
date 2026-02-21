<script>
  import Juggler from './Juggler.svelte';
  import PeertubeVideoPlayer from './video/PeertubeVideoPlayer.svelte';
  import * as api from '$lib/peertube-openapi';

  /**
   * @typedef {Object} Props
   * @property {api.Video[]} videos
   * @property {boolean} autoplay
   */

  /** @type {Props} */
  let { videos, autoplay } = $props();
  let juggler = $state();
  let currentIndex = $state(0);
  const videoPlayers = $derived(
    videos.map((video) => {
      return {
        id: video.id,
        aspectRation: video.aspectRatio,
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
    const currentVideo = videoPlayers[currentIndex];
    if (autoplay && currentVideo.player) {
      currentVideo.player.play();
      currentVideo.player.addEventListener('playbackStatusUpdate', nextOnEnded);
    }

    const prevIdx = (currentIndex + videos.length - 1) % videos.length;
    const nextIdx = (currentIndex + 1) % videos.length;
    if (prevIdx !== currentIndex && videoPlayers[prevIdx].player) {
      videoPlayers[prevIdx].player.pause();
      videoPlayers[prevIdx].player.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
    if (nextIdx !== currentIndex && videoPlayers[nextIdx].player) {
      videoPlayers[nextIdx].player.pause();
      videoPlayers[nextIdx].player.removeEventListener(
        'playbackStatusUpdate',
        nextOnEnded
      );
    }
  });

  /** @param {number} index */
  function onIndexChanged(index) {
    videoPlayers[index].player?.forceLoad();
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
        aspectRatio={video.aspectRatio}
      />
    </div>
  {/snippet}
</Juggler>
