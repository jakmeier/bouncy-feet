<script>
  import TrackAudio from '$lib/components/TrackAudio.svelte';
  import { setContext } from 'svelte';

  let track = '';

  /** @param {string} newTrack */
  async function setTrack(newTrack) {
    track = newTrack;
  }

  setContext('music', { setTrack });
</script>

<TrackAudio {track} />
<slot />
