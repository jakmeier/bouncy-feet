<script>
  import { t, locale, dateLocale } from '$lib/i18n';
  import { getContext, onDestroy, onMount } from 'svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { bpm } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import { Course } from '$lib/instructor/bouncy_instructor';
  import LogoHeader from '../ui/LogoHeader.svelte';
  import { formatDuration, intervalToDuration } from 'date-fns';
  import TrackerPreview from '../avatar/TrackerPreview.svelte';
  import Background from '../ui/sections/Background.svelte';

  /**
   * @typedef {Object} Props
   * @property {Course} course
   * @property {number} lessonIndex
   * @property {function} onDone
   */

  /** @type {Props}*/
  let { course, lessonIndex, onDone } = $props();

  const { stopTrack, setTrack, songTitle, songAuthor } = getContext('music');
  const { tracker } = getContext('tracker');

  let lesson = course.lessons[lessonIndex];
  /** @type {string} */
  let title = course.lessons[lessonIndex].name;
  let lessonDescription = course.lessons[lessonIndex].explanation;

  let isVideoOpen = $state(writable(false));
  let size = 100;

  const songList = songs.list();
  let trackIndex = $state(0);
  /** @param {number} index */
  function changeTrack(index) {
    const track = songList[index % songList.length];
    if (track) {
      setTrack(track.id);
    }
  }

  /** @type {import('date-fns').FormatDurationOptions} */
  const formatOpts = $derived({
    ...dateLocale($locale),
  });
  /** @type {import('date-fns').Duration} */
  let trainingDuration = $derived(
    intervalToDuration({ start: 0, end: tracker.duration() })
  );
  let trainingBeats = $derived(tracker.trackedSubbeats / 2);

  onMount(() => {
    setTrack(songList[0].id);
  });
  onDestroy(() => {
    stopTrack();
  });
</script>

<Background bgColor="var(--theme-main)" color="var(--theme-black)" />

<LogoHeader mainColor {title} />

<div class="description">
  {lessonDescription}
</div>

<div class="preview">
  <TrackerPreview {tracker} size={200} />
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
  <!-- TODO: translated texts, data from actual course lesson -->
  <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
  <div>easy difficulty</div>
  <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
  <div>medium energy</div>
</div>

<div class="controls">
  <button onclick={onDone}>{$t('courses.lesson.start-button')}</button>
</div>

<DarkSection>
  <h3>{$t('courses.lesson.settings-subtitle')}</h3>
  <div class="about-lesson">
    {#if lesson.video && lesson.video.length > 0}
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

  <Footer white />
</DarkSection>

<Popup bind:isOpen={isVideoOpen} showOkButton>
  <div class="video-wrapper">
    {#if lesson.video && lesson.video.length > 0}
      <Video path={`${base}${lesson.video}`}></Video>
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
