<script>
  import Area from '$lib/components/ui/Area.svelte';
  import { t } from '$lib/i18n';
  import { beatCounter } from '$lib/stores/Beat';
  import { derived } from 'svelte/store';
  import DanceAnimation from './DanceAnimation.svelte';
  import { getContext } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} dance
   */

  /** @type {Props} */
  let { dance } = $props();

  /** @type {LocalState}*/
  const localState = getContext('localState');

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
  <div>
    {#if dance}
      <!-- <AvatarStyleContext
        headStyle={{ shape: 'square', size: 0.8, strokeWidth: 1.2 }}
        bodyShape={{ strokeWidth: 1.5 }}
        coloring={ORANGE_COLORING}
      > -->
      <DanceAnimation
        size={trainingsWidth}
        {dance}
        beatDelay={3}
        hiddenBeats={3}
        showOverflow={true}
        beat={shownBeat}
      />
      <!-- </AvatarStyleContext> -->
    {:else}
      <Area
        width="{trainingsWidth}px"
        height="{trainingsWidth}px"
        borderRadius="20px"
        borderWidth="0px"
      ></Area>
    {/if}
  </div>
  <div class="buttons">
    <a href="./coach/{localState.selectedCoach}">
      <button>
        {$t('home.continue-learning-button')}
      </button>
    </a>
    <a href="./coach/select">
      <button>
        {$t('home.change-teacher-button')}
      </button>
    </a>
    <a href="./firstCourse">
      <button>
        {$t('home.tutorial-button')}
      </button>
    </a>
  </div>
</div>

<style>
  .container {
    position: relative;
  }

  button {
    margin: 1rem auto;
  }

  .buttons {
    display: flex;
    flex-direction: column;
  }
</style>
