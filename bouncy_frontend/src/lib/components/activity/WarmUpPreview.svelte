<script>
  import { t, locale, dateLocale } from '$lib/i18n';
  import { formatDuration, intervalToDuration } from 'date-fns';
  import { getContext, onDestroy, onMount } from 'svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { bpm } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import TrackerPreview from '../avatar/TrackerPreview.svelte';
  import MusicVolumeControl from '../audio/MusicVolumeControl.svelte';
  import { base } from '$app/paths';

  /**
   * @typedef {Object} Props
   * @property {string} videoUrl
   * @property {string} description
   * @property {boolean} audioControl
   * @property {string} trackId
   * @property {()=>void} onDone
   */

  /** @type {Props} */
  let { videoUrl, description, audioControl, trackId, onDone } = $props();

  const { setTrack, stopTrack, songTitle, songAuthor } = getContext('music');
  let { tracker } = getContext('tracker');

  let isVideoOpen = $state(writable(false));

  /** @type {import('date-fns').FormatDurationOptions} */
  const formatOpts = $derived({
    ...dateLocale($locale),
  });

  const songList = songs.list();
  let trackIndex = $state(0);
  function changeTrack(index) {
    const track = songList[index % songList.length];
    if (track) {
      setTrack(track.id);
    }
  }
  /** @type {import('date-fns').Duration} */
  let trainingDuration = $derived(
    intervalToDuration({ start: 0, end: tracker.duration() })
  );
  let trainingBeats = $derived(tracker.trackedSubbeats / 2);

  onMount(() => {
    setTrack(trackId);
  });
  onDestroy(() => {
    stopTrack();
  });
</script>

<StandardPage title={$t('record.warmup-preview-title')} mainColor>
  <div class="description">
    {description}
  </div>

  <div class="preview">
    <div class="exercise-part">
      <TrackerPreview {tracker} />
    </div>
  </div>

  <div class="overview">
    <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
    <div>
      {formatDuration(trainingDuration, formatOpts)}
    </div>
    <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
    <div>
      {trainingBeats}
      {$t('courses.lesson.num-beats-label')} @
      {$bpm} bpm
    </div>
  </div>

  <div class="controls">
    <button onclick={onDone}>{$t('courses.lesson.start-button')}</button>
    <MusicVolumeControl />
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

  .overview,
  .about-lesson,
  .description {
    margin: 2em 0em 3rem;
  }

  .overview,
  .about-lesson {
    display: grid;
    grid-template-columns: auto auto;
    gap: 1rem;
    align-items: center;
    justify-content: left;
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

  .overview img {
    width: 2rem;
    height: 2rem;
  }
</style>
