<script>
  import { goto } from '$app/navigation';
  import BeatVisualizer from '$lib/components/record/BeatVisualizer.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import BackHeaderWithLogo from '$lib/components/ui/header/BackHeaderWithLogo.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import { t } from '$lib/i18n';
  import {
    bpm,
    setBeatStart,
    setBpm,
    beatStart,
    beatDuration,
  } from '$lib/stores/Beat';

  let start = performance.now();
  let lastTap = performance.now();

  let counter = $state(-1);
  let bpmSelected = $derived(counter > 0);

  const timeout = 2000;

  let displayedBpm = $derived(bpmSelected ? $bpm : '?');

  function reset() {
    counter = -1;
    setBeatStart(performance.now());
  }

  function tap() {
    const now = performance.now();
    if (lastTap + timeout < now) {
      counter = -1;
    }
    lastTap = now;
    setBeatStart(lastTap);

    if (counter === -1) {
      start = lastTap;
    }
    counter += 1;

    if (counter > 2) {
      setBpm(Math.round((counter * 60_000) / (lastTap - start)));
    }
  }
</script>

<NightSection fillScreen>
  <BackHeaderWithLogo
    title={$t('editor.video.detect-bpm-button')}
    onBack={() => goto('/')}
    black
  ></BackHeaderWithLogo>

  <div class="content">
    <div class="bpm-container">
      <div class="bpm">
        {displayedBpm}
      </div>

      <div>
        {$t('record.estimated-bpm-label')}
      </div>

      <div class="reset-button">
        <Button
          class="reset"
          on:click={reset}
          symbolClass="thin"
          symbolSize={28}
          symbol="cancel"
        ></Button>
      </div>
    </div>

    <div class="visualizer" onpointerdown={tap}>
      <BeatVisualizer
        size={200}
        accentColor
        start={$beatStart}
        timeBetweenBeats={$beatDuration}
      >
        {$t('record.bpm-tap-button')}
      </BeatVisualizer>
    </div>
  </div>
  <Footer white></Footer>
</NightSection>

<style>
  .content {
    margin: auto;
  }

  .bpm-container {
    display: grid;
    text-align: center;
    justify-content: center;
    justify-items: center;
    width: 200px;
    min-height: 6rem;
    border-radius: 20px;
    margin: 2rem auto;
  }
  .bpm {
    font-size: var(--font-large);
  }
  .visualizer {
    padding-top: 20px;
  }
  .reset-button {
    height: 2rem;
    margin: 1rem;
  }
</style>
