<script>
  import { base } from '$app/paths';
  import { getContext } from 'svelte';

  let musicContext = getContext('music');
  let { stopTrack, resumeTrack, setVolume } = musicContext;
  let musicEnabled = $derived(musicContext.gain > 0 && musicContext.trackOn);

  /** @type {Props} */
  let { color = 'var(--theme-neutral-black)' } = $props();

  function toggleMusic() {
    if (musicEnabled) {
      stopTrack();
    } else {
      resumeTrack();
      if (musicContext.gain <= 0) {
        setVolume(0.5);
      }
    }
  }

  function updateVolume(event) {
    const target = event.target;
    const newVol = parseFloat(target.value);
    setVolume(newVol);
    if (newVol > 0) {
      resumeTrack();
    }
  }
</script>

<div class="music-control" style="--component-color: {color}">
  <button class="toggle" onclick={toggleMusic}>
    {#if musicEnabled}
      <img
        src="{base}/img/symbols/bf_audio_vol.svg"
        alt="audio_volume"
        class:inverted={color != 'var(--theme-neutral-black)'}
      />
    {:else}
      <img
        src="{base}/img/symbols/bf_audio_vol_off.svg"
        alt="audio_volume_off"
        class:inverted={color != 'var(--theme-neutral-black)'}
      />
    {/if}
  </button>
  <div class="volume-slider">
    <div class="slider-wrapper">
      <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        value={musicContext.gain}
        oninput={updateVolume}
      />
      <div class="slider-markers">
        {#each Array.from({ length: 5 }, (_, i) => i / 4) as marker}
          <div class="marker" style="left: {marker * 100}%"></div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .music-control {
    display: grid;
    grid-template-columns: 4rem auto;
    align-items: center;
    justify-items: start;
    gap: 0.5rem;
  }

  .toggle {
    width: 4rem;
    height: 4rem;
    border: none;
    background-color: transparent;
    cursor: pointer;
    padding: 0;
    margin: 0;
    min-width: 0;
  }

  .toggle img {
    width: 4rem;
    height: 4rem;
  }

  .volume-slider {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    position: relative;
  }

  .slider-wrapper {
    position: relative;
    flex: 1;
    height: 2rem;
  }

  input[type='range'] {
    position: absolute;
    /* (2rem - 8px) / 2 */
    top: 12px;
    left: 0;
    width: 100%;
    height: 8px;
    border-radius: 4px;
    appearance: none;
    z-index: 1;
    background-color: var(--component-color);
  }

  input[type='range']::-webkit-slider-thumb {
    appearance: none;
    width: 2rem;
    height: 2rem;
    background: var(--theme-accent);
    border-radius: 50%;
    border: none;
    cursor: pointer;
    /* margin-top: -0.35rem; */
    z-index: 2;
  }

  input[type='range']::-moz-range-thumb {
    width: 2rem;
    height: 2rem;
    background: var(--theme-accent);
    border: none;
    border-radius: 50%;
    cursor: pointer;
  }

  .slider-markers {
    position: absolute;
    /* (2rem - 8px) / 2 */
    top: 12px;
    left: 10px;
    width: calc(100% - 20px);
    height: 100%;
    pointer-events: none;
    z-index: 0;
  }

  .slider-markers .marker {
    position: absolute;
    height: 2rem;
    top: -10px;
    width: 6px;
    border-radius: 3px;
    background: var(--component-color);
  }

  .inverted {
    filter: invert(100%);
  }
</style>
