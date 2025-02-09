<script>
  import Area from '$lib/components/ui/Area.svelte';
  import { EXAMPLE_CUSTOM_COLORING } from '$lib/constants';
  import { t } from '$lib/i18n';
  import { beatCounter } from '$lib/stores/Beat';
  import { derived } from 'svelte/store';
  import DanceAnimation from './DanceAnimation.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper} */
  export let dance;

  /**
   * @type {number | undefined}
   */
  let trainingsWidth = 250;

  let firstMovement = $beatCounter + 5;
  let storedBeat = 0;
  let lastExternalBeat = firstMovement;
  let shownBeat = derived([beatCounter], ([$beatCounter]) => {
    const beatDiff = $beatCounter - lastExternalBeat;
    if (beatDiff > 0) {
      if (!freezeDancer(lastExternalBeat - firstMovement)) {
        storedBeat += beatDiff;
      }
      lastExternalBeat = $beatCounter;
    }
    return storedBeat;
  });

  function freezeDancer(beat) {
    // stop dancing after some time
    if (beat >= 120) return true;

    return beat % 20 >= 12;
  }
</script>

<div class="container">
  <div class="half-transparent">
    {#if dance}
      <DanceAnimation
        size={trainingsWidth}
        {dance}
        style={EXAMPLE_CUSTOM_COLORING}
        beatDelay={3}
        hiddenBeats={3}
        showOverflow={true}
        beat={shownBeat}
      />
    {:else}
      <Area
        width="{trainingsWidth}px"
        height="{trainingsWidth}px"
        borderRadius="20px"
        borderWidth="0px"
      ></Area>
    {/if}
  </div>
  <div class="overlay">
    <a href="./coach/chorok">
      <div class="accent button">
        {$t('home.continue-learning-button')}
      </div>
    </a>
    <a href="./coach/juhwang">
      <div class="lime button">
        {$t('home.change-teacher-button')}
      </div>
    </a>
  </div>
</div>

<style>
  .container {
    position: relative;
  }

  .half-transparent {
    opacity: 0.38;
  }

  .accent {
    background-color: var(--theme-accent);
  }
  .lime {
    background-color: var(--theme-main);
  }
  .button {
    text-decoration: none;
    border-radius: 999px;
    width: fit-content;
    padding: 10px 20px;
    font-size: var(--font-normal);
    margin: 30px auto;
  }

  .overlay {
    position: absolute;
    top: 25%;
    left: 0;
    text-align: center;
  }
</style>
