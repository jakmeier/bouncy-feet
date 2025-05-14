<script>
  import TrackAudio from '$lib/components/audio/TrackAudio.svelte';
  import { setChannelGain } from '$lib/stores/Audio';
  import { songs } from '$lib/stores/Songs';
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /** @type {Song | undefined} */
  let track = $state(undefined);
  /** @type {TrackAudio | undefined} */
  let audio = $state();

  let titleSetter = (_) => {};
  const songTitle = readable('-', (set) => {
    titleSetter = set;
  });

  let authorSetter = (_) => {};
  const songAuthor = readable('-', (set) => {
    authorSetter = set;
  });

  let gain = $state(1.0);

  /** @param {string} newTrackId */
  async function setTrack(newTrackId) {
    track = songs.get(newTrackId);
    titleSetter(track?.title || '-');
    authorSetter(track?.author || '-');
  }

  function resetTrack() {
    audio?.resetTrack();
  }

  function stopTrack() {
    audio?.stopMusic();
  }

  function resumeTrack() {
    audio?.resumeMusic();
  }

  /** @param {number} vol */
  function setVolume(vol) {
    gain = vol;
  }

  setContext('music', {
    setTrack,
    resetTrack,
    stopTrack,
    resumeTrack,
    setVolume,
    songTitle,
    songAuthor,
    get gain() {
      return gain;
    },
    get trackOn() {
      return audio?.trackIsOn();
    },
  });

  $effect(() => {
    setChannelGain('music', gain);
  });
</script>

{#if track}
  <TrackAudio bind:this={audio} {track} />
{/if}

{@render children?.()}
