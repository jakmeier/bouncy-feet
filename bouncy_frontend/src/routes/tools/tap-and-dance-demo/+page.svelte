<script>
  import { asset } from '$app/paths';
  import { onDestroy } from 'svelte';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import { TapDanceDemoAudioEngine } from '$lib/tools/tap-and-dance/TapDanceDemoAudioEngine.js';

  /** @typedef {import('$lib/tools/tap-and-dance/TapDanceDemoAudioEngine.js').SoundId} SoundId */
  /** @type {{id: SoundId, name: string, icon: string, color: string}[]} */
  const instruments = [
    {
      id: 'kick',
      name: 'Kick',
      icon: 'radio_button_checked',
      color: 'var(--theme-accent)',
    },
    {
      id: 'clap',
      name: 'Clap',
      icon: 'back_hand',
      color: 'var(--theme-neutral-white)',
    },
    {
      id: 'hat',
      name: 'Hi-hat',
      icon: 'graphic_eq',
      color: 'var(--theme-main)',
    },
  ];

  let bpm = $state(124);
  let enabled = $state({ kick: true, clap: true, hat: true });
  let rampEnabled = $state(false);
  let startBpm = $state(100);
  let endBpm = $state(130);
  let rampDuration = $state(20);
  /** @type {4 | 8} */
  let rampEveryBeats = $state(8);
  let stopAfterDuration = $state(false);
  let currentBpm = $state(124);
  let playing = $state(false);
  let starting = $state(false);
  let activeStep = $state(-1);
  let audioError = $state('');
  const rampLocked = $derived(playing || starting);

  const audioEngine = new TapDanceDemoAudioEngine(
    {
      kick: asset('/audio/house_909_kick.mp3'),
      clap: asset('/audio/classic-clap.mp3'),
      // This is also the original house kit's hi-hat sample.
      hat: asset('/audio/dnb_hihat.mp3'),
    },
    () => ({
      bpm,
      enabled,
      speedRamp: rampEnabled
        ? {
            startBpm,
            endBpm,
            duration: rampDuration,
            everyBeats: rampEveryBeats,
            stopAfterDuration,
          }
        : null,
    }),
    (step, tempo) => {
      activeStep = step;
      currentBpm = tempo;
    },
    () => {
      playing = false;
      activeStep = -1;
    }
  );

  /** @param {string | number} value @param {number} min @param {number} max @param {number} fallback */
  function boundedInteger(value, min, max, fallback) {
    const number = Number(value);
    return value === '' || !Number.isFinite(number)
      ? fallback
      : Math.min(max, Math.max(min, Math.round(number)));
  }

  /** @param {string | number} value */
  function setBpm(value) {
    bpm = boundedInteger(value, 50, 220, bpm);
  }

  function stopPlaying() {
    audioEngine.stop();
    playing = false;
    activeStep = -1;
  }

  async function togglePlaying() {
    if (starting) return;
    if (playing) {
      stopPlaying();
      return;
    }
    starting = true;
    audioError = '';
    try {
      playing = await audioEngine.start();
    } catch (error) {
      audioEngine.stop();
      console.error('Could not start Tap & Dance demo audio', error);
      audioError = 'Could not start audio. Please try again.';
    } finally {
      starting = false;
    }
  }

  onDestroy(() => audioEngine.dispose());
</script>

<svelte:document
  onvisibilitychange={() => {
    if (document.hidden) stopPlaying();
  }}
/>
<svelte:window onpagehide={stopPlaying} />

<svelte:head>
  <title>Tap & Dance Demo | Bouncy Feet</title>
  <meta
    name="description"
    content="Tap sounds on or off and dance to a house beat. Set your tempo or gradually increase the speed."
  />
</svelte:head>

<NightSection fillScreen>
  <LogoHeader title="Tap & Dance" black />

  <main class="tool">
    <section class="tempo" aria-label="Tempo">
      <div class="tempo-heading">BPM</div>
      <div class="bpm-value">
        {rampEnabled ? (playing ? currentBpm.toFixed(1) : startBpm) : bpm}
      </div>
      {#if !rampEnabled}
        <input
          aria-label="Beats per minute"
          type="range"
          min="50"
          max="220"
          step="1"
          value={bpm}
          oninput={(event) => setBpm(event.currentTarget.value)}
        />
        <div class="tempo-actions">
          <button onclick={() => setBpm(bpm - 5)}>-5</button>
          <button onclick={() => setBpm(bpm - 1)}>-1</button>
          <button onclick={() => setBpm(bpm + 1)}>+1</button>
          <button onclick={() => setBpm(bpm + 5)}>+5</button>
        </div>
      {/if}

      <div class="ramp-toggle">
        <Toggle
          bind:isOn={rampEnabled}
          disabled={rampLocked}
          ariaLabel="Increase speed during exercise"
          border
        />
        <span>Increase speed during exercise</span>
      </div>
      {#if rampEnabled}
        <fieldset class="ramp-fields" disabled={rampLocked}>
          <legend class="sr-only">Speed ramp</legend>
          <label>
            Start BPM
            <input
              type="number"
              min="50"
              max="220"
              step="1"
              value={startBpm}
              onchange={(event) => {
                startBpm = boundedInteger(
                  event.currentTarget.value,
                  50,
                  220,
                  startBpm
                );
                endBpm = Math.max(startBpm, endBpm);
                event.currentTarget.value = String(startBpm);
              }}
            />
          </label>
          <label>
            End BPM
            <input
              type="number"
              min={startBpm}
              max="220"
              step="1"
              value={endBpm}
              onchange={(event) => {
                endBpm = boundedInteger(
                  event.currentTarget.value,
                  startBpm,
                  220,
                  endBpm
                );
                event.currentTarget.value = String(endBpm);
              }}
            />
          </label>
          <label>
            Duration (s)
            <input
              type="number"
              min="1"
              max="3600"
              step="1"
              value={rampDuration}
              onchange={(event) => {
                rampDuration = boundedInteger(
                  event.currentTarget.value,
                  1,
                  3600,
                  rampDuration
                );
                event.currentTarget.value = String(rampDuration);
              }}
            />
          </label>
          <label>
            Increase every
            <select bind:value={rampEveryBeats}>
              <option value={4}>4 beats</option>
              <option value={8}>8 beats</option>
            </select>
          </label>
          <div class="ramp-toggle stop-toggle">
            <Toggle
              bind:isOn={stopAfterDuration}
              disabled={rampLocked}
              ariaLabel="Stop after duration"
              border
            />
            <span>Stop after duration</span>
          </div>
        </fieldset>
        <p class="ramp-help">
          {#if stopAfterDuration}
            Stops after {rampDuration}s.
          {:else}
            Reaches {endBpm} BPM at the first {rampEveryBeats}-beat boundary at
            or after {rampDuration}s, then holds.
          {/if}
          Stop to edit or restart.
        </p>
      {/if}
    </section>

    <p class="hint">Tap a sound to turn it on or off.</p>
    <section class="instrument-grid" aria-label="Instruments">
      {#each instruments as instrument}
        <button
          class="instrument"
          class:on={enabled[instrument.id]}
          style:--instrument-color={instrument.color}
          aria-label={instrument.name}
          aria-pressed={enabled[instrument.id]}
          onclick={() =>
            (enabled = {
              ...enabled,
              [instrument.id]: !enabled[instrument.id],
            })}
        >
          <span
            class="instrument-icon material-symbols-outlined"
            translate="no"
            aria-hidden="true">{instrument.icon}</span
          >
          <strong>{instrument.name}</strong>
          <small>{enabled[instrument.id] ? 'On' : 'Off'}</small>
        </button>
      {/each}
    </section>

    <div class="transport">
      <button
        class="play"
        onclick={togglePlaying}
        disabled={starting}
        aria-label={starting
          ? 'Loading beat'
          : playing
            ? 'Stop beat'
            : 'Play beat'}
      >
        <img
          src={asset(`/img/symbols/${playing ? 'bf_stop.svg' : 'bf_play.svg'}`)}
          alt=""
        />
      </button>
    </div>
    {#if audioError}<p class="audio-error" role="alert">{audioError}</p>{/if}

    <div class="timeline" aria-label="8 beat loop">
      <span class="sr-only" aria-live="polite">
        {activeStep >= 0
          ? `Beat ${Math.floor(activeStep / 2) + 1}${activeStep % 2 ? ' and' : ''}`
          : playing
            ? 'Starting beat'
            : 'Beat stopped'}
      </span>
      {#each Array(8).keys() as beat}
        <div class="beat">
          <span class="pulse" class:active={activeStep === beat * 2}></span>
          <span class="and" class:active={activeStep === beat * 2 + 1}></span>
          <b>{beat + 1}</b>
        </div>
      {/each}
    </div>
  </main>
</NightSection>

<style>
  .tool {
    width: min(100%, 680px);
    margin: 0 auto;
    padding-bottom: 2rem;
    color: var(--theme-neutral-white);
  }
  .tempo,
  .instrument {
    border: 1px solid var(--theme-neutral-darker-gray);
    border-radius: 20px;
    background: linear-gradient(
      145deg,
      var(--theme-neutral-almost-black),
      var(--theme-neutral-black)
    );
  }
  .tempo {
    padding: 1rem 1.2rem;
  }
  .tempo-heading {
    color: var(--theme-neutral-dark-gray);
    letter-spacing: 0.12em;
  }
  .bpm-value {
    color: var(--theme-main);
    font-size: calc(var(--font-large) * 2);
    line-height: 1;
    text-align: center;
    font-variant-numeric: tabular-nums;
  }
  input[type='range'] {
    width: 100%;
    max-width: none;
    accent-color: var(--theme-main);
  }
  .tempo-actions {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.55rem;
    margin-top: 0.7rem;
  }
  .tempo-actions button {
    min-width: 0;
    width: 100%;
    min-height: 44px;
    padding: 0.55rem;
    border: 1px solid var(--theme-neutral-darker-gray);
    color: var(--theme-neutral-white);
    background: var(--theme-neutral-almost-black);
    font-size: var(--font-normal);
  }
  .ramp-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
    font-size: var(--font-small);
  }
  .ramp-fields {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.6rem;
    padding: 0;
    margin: 1rem 0 0;
    border: 0;
  }
  .ramp-fields label {
    min-width: 0;
    font-size: var(--font-small);
  }
  .ramp-fields input,
  .ramp-fields select {
    box-sizing: border-box;
    width: 100%;
    min-width: 0;
    min-height: 44px;
    margin-top: 0.3rem;
    color: var(--theme-neutral-white);
    background: var(--theme-neutral-almost-black);
    border: 1px solid var(--theme-neutral-darker-gray);
    border-radius: 8px;
    padding: 0.5rem;
  }
  .stop-toggle {
    grid-column: 1 / -1;
  }
  .ramp-help,
  .hint,
  .instrument small {
    color: var(--theme-neutral-dark-gray);
    font-size: var(--font-small);
  }
  .hint {
    text-align: center;
    margin: 1.2rem 0 0.7rem;
  }
  .instrument-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.75rem;
  }
  .instrument {
    display: flex;
    min-width: 0;
    width: 100%;
    min-height: 145px;
    padding: 0.7rem;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--theme-neutral-dark-gray);
  }
  .instrument.on {
    border-color: var(--instrument-color);
    color: var(--instrument-color);
    box-shadow: inset 0 0 22px
      color-mix(in srgb, var(--instrument-color) 11%, transparent);
  }
  .instrument-icon {
    font-size: var(--font-large);
    line-height: 1;
  }
  .instrument strong {
    margin-top: 0.5rem;
    font-size: var(--font-normal);
    text-transform: uppercase;
  }
  .transport {
    display: flex;
    justify-content: center;
    padding: 1.2rem 0;
  }
  .play {
    min-width: 74px;
    width: 74px;
    height: 74px;
    padding: 0;
    background: transparent;
  }
  .play img {
    display: block;
    width: 100%;
    height: 100%;
  }
  .audio-error {
    color: var(--theme-accent-medium);
    text-align: center;
  }
  .timeline {
    display: flex;
    gap: 0.3rem;
    padding: 0.8rem;
    border: 1px solid var(--theme-neutral-darker-gray);
    border-radius: 15px;
    background: var(--theme-neutral-black);
  }
  .beat {
    display: grid;
    grid-template-columns: 1fr 1fr;
    flex: 1;
    align-items: end;
    gap: 3px;
    color: var(--theme-neutral-dark-gray);
  }
  .beat b {
    grid-column: 1 / -1;
    margin-top: 0.35rem;
    font-size: var(--font-small);
    font-weight: normal;
  }
  .pulse,
  .and {
    display: block;
    height: 20px;
    border-radius: 999px;
    background: var(--theme-neutral-darker-gray);
    transition: 0.06s;
  }
  .and {
    height: 10px;
  }
  .pulse.active,
  .and.active {
    background: var(--theme-main);
    transform: scaleY(1.25);
  }
  .tool button:focus-visible,
  .tool input:focus-visible,
  .tool select:focus-visible {
    outline: 3px solid var(--theme-accent);
    outline-offset: 3px;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
  @media (prefers-reduced-motion: reduce) {
    .pulse,
    .and {
      transition: none;
    }
  }
  @media (max-width: 600px) {
    .tool {
      margin-top: -0.5rem;
    }
    .instrument-grid {
      gap: 0.5rem;
    }
    .instrument {
      min-height: 125px;
    }
  }
</style>
