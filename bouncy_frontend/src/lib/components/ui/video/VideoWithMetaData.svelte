<script>
  import VideoPlayer from './VideoPlayer.svelte';

  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").VideoDef} video
   */

  /** @type {Props} */
  let { video } = $props();

  /** @type {number[]} */
  let beats = $derived(Array.from(video.beats()));
  let markers = $derived.by(() => {
    let out = [];
    for (var t of video.startMarkers()) {
      out.push({ time: t, label: 'start', icon: 'logo' });
    }
    // TODO: step markers
    return out;
  });
</script>

{#if video && !video.isEmpty()}
  <VideoPlayer path={video.path()} {beats} {markers}></VideoPlayer>
{/if}
