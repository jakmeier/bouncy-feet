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
    const prev = (currentIndex + ids.length - 1) % ids.length;
    const next = (currentIndex + 1) % ids.length;
    if (prev !== currentIndex) {
      videos[prev].player?.pause();
    }
    if (next !== currentIndex) {
      videos[next].player?.pause();
    }
  });

  function next() {
    currentIndex = (currentIndex + 1) % ids.length;
  }

  function pos(index) {
    if (index < currentIndex) {
      return -150;
    }
    if (index > currentIndex) {
      return 150;
    }
    return 0;
  }
</script>

<div class="container">
  {#each videos as video, index}
    <JuggleElement x={pos(index)}>
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
