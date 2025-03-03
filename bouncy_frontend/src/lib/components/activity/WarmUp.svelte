<script>
  import { t } from '$lib/i18n.js';
  import { getContext } from 'svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { beatCounter, bpm, timeBetweenMoves } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import Step from '../../../routes/collection/Step.svelte';
  import StandardPage from '../ui/StandardPage.svelte';

  /**
   * @typedef {Object} Props
   * @property {StepWrapper} step
   * @property {string} videoUrl
   * @property {string} description
   * @property {boolean} audioControl
   */

  /** @type {Props} */
  let { step, videoUrl, description, audioControl } = $props();

  const { setTrack, songTitle, songAuthor } = getContext('music');

  let isVideoOpen = $state(writable(false));
  let size = 100;

  const songList = songs.list();
  let trackIndex = $state(0);
  function changeTrack(index) {
    const track = songList[index % songList.length];
    if (track) {
      setTrack(track.id);
    }
  }
</script>

<LightBackground />

<!-- TODO: translated title -->
<StandardPage title="Warm-up" white>
  <!-- TODO: translated texts -->
  <div class="description">
    {description}
  </div>

  <div class="background-strip">
    <div class="preview">
      <div class="exercise-part">
        <Step
          {step}
          poseIndex={$beatCounter}
          animationTime={$timeBetweenMoves * 0.7}
          {size}
        ></Step>
      </div>
    </div>
  </div>

  <!-- TODO: translated texts with real data -->
  <div class="overview">
    <div>Duration</div>
    <div>1 min</div>
  </div>

  <!-- TODO: fix link -->
  <div class="controls">
    <a href="./record">
      <button>{$t('courses.lesson.start-button')}</button>
    </a>
  </div>

  {#if audioControl}
    <DarkSection>
      <h3>{$t('courses.lesson.settings-subtitle')}</h3>
      <div class="about-lesson">
        {#if videoUrl && videoUrl.length > 0}
          <button class="action big-col" onclick={() => ($isVideoOpen = true)}
            >{$t('courses.lesson.to-video-button')}</button
          >
        {/if}
        <div class="left">{$songTitle} {$t('music.by')} {$songAuthor}</div>
        <button
          class="action right"
          onclick={() => {
            trackIndex += 1;
            changeTrack(trackIndex);
          }}>{$t('courses.lesson.next-song-button')}</button
        >
        <div class="left">{$t('courses.lesson.bpm-label')}</div>
        <div class="left">{$bpm} BPM</div>
      </div>
    </DarkSection>
  {/if}
</StandardPage>

<Popup bind:isOpen={isVideoOpen} showOkButton>
  <div class="video-wrapper">
    {#if videoUrl && videoUrl.length > 0}
      <Video path={videoUrl}></Video>
    {/if}
  </div>
</Popup>

<style>
  h3 {
    margin: -0.5rem;
  }

  .preview {
    display: flex;
    justify-content: space-around;
  }

  .exercise-part {
    text-align: center;
  }

  .controls {
    text-align: center;
    margin: 2rem auto 5rem;
  }

  .background-strip {
    margin: 10px -100%;
    padding: 2rem;
    background-color: var(--theme-main);
    rotate: 8deg;
  }
  .background-strip .preview {
    rotate: -8deg;
  }

  .overview,
  .about-lesson,
  .description {
    margin: 2em 0em 3rem;
  }

  .overview,
  .about-lesson {
    display: grid;
    grid-template-columns: 1fr max-content;
    gap: 1rem;
    align-items: center;
  }

  .about-lesson > .left {
    justify-self: start;
  }
  .about-lesson > .right {
    justify-self: end;
  }

  .about-lesson button {
    justify-self: center;
    width: 100%;
  }

  .big-col {
    grid-column-start: 1;
    grid-column-end: 3;
    margin: 1rem;
  }

  .video-wrapper {
    width: 100vw;
  }

  @media (min-width: 730px) {
    .background-strip {
      rotate: 4deg;
    }
    .background-strip .preview {
      rotate: -4deg;
    }
  }
</style>
