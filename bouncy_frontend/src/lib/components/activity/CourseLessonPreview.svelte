<script>
  import { t } from '$lib/i18n.js';
  import { getContext } from 'svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { beatCounter, bpm, timeBetweenMoves } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import { Course } from 'bouncy_instructor';
  import Step from '../../../routes/collection/Step.svelte';
  import LogoHeader from '../ui/LogoHeader.svelte';

  const { setTrack, songTitle, songAuthor } = getContext('music');

  /**
   * @typedef {Object} Props
   * @property {Course} course
   * @property {number} lessonIndex
   * @property {function} onDone
   */

  /** @type {Props}*/
  let { course, lessonIndex, onDone } = $props();

  let lesson = course.lessons[lessonIndex];
  /** @type {string} */
  let title = course.lessons[lessonIndex].name;
  let lessonDescription = course.lessons[lessonIndex].explanation;

  /** @type {number | undefined} */
  let partIndex;
  /** @type {import("bouncy_instructor").LessonPart | undefined} */
  let exercise;

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
</script>

<LightBackground />

<LogoHeader white {title} />

<div class="description">
  {lessonDescription}
</div>

<div class="background-strip">
  <div class="preview">
    <!-- <h3>{$t('courses.lesson.steps-subtitle')}</h3> -->
    {#each lesson.parts as part, index}
      <div class="exercise-part">
        <Step
          step={part.step}
          poseIndex={$beatCounter}
          animationTime={$timeBetweenMoves * 0.7}
          {size}
        ></Step>
        <b>4x</b>
      </div>
      {#if index !== lesson.parts.length - 1}
        <div class="arrow">→</div>
      {/if}
    {/each}
  </div>
</div>

<!-- TODO: translated texts, data from actual course lesson -->
<div class="overview">
  <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
  <div>easy difficulty</div>
  <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
  <div>medium energy</div>
  <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
  <div>1 min</div>
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

  .exercise-part {
    text-align: center;
  }

  .arrow {
    line-height: 100px;
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

  @media (min-width: 730px) {
    .background-strip {
      rotate: 4deg;
    }
    .background-strip .preview {
      rotate: -4deg;
    }
  }
</style>
