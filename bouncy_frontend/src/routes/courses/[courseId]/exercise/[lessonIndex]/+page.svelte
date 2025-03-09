<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext } from 'svelte';
  import Step from '../../../../collection/Step.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { beatCounter, bpm, timeBetweenMoves } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';

  const { getCourse } = getContext('courses');
  const { setTrack, songTitle, songAuthor } = getContext('music');

  let id;
  /** @type {import('$lib/instructor/bouncy_instructor').Course } */
  let course;
  /** @type {number} */
  let lessonIndex;
  /** @type { import('$lib/instructor/bouncy_instructor').Lesson } */
  let lesson = $state();
  /** @type {number | undefined} */
  let partIndex;
  /** @type {import("$lib/instructor/bouncy_instructor").LessonPart | undefined} */
  let exercise;
  /** @type {boolean} */
  let done;
  /** @type {string} */
  let title = $state();

  let isVideoOpen = $state(writable(false));
  let size = 100;

  function loadCourse() {
    id = page.params.courseId;
    course = getCourse(id);
    lessonIndex = Number.parseInt(page.params.lessonIndex);
    lesson = course.lessons[lessonIndex];
    title = $t('courses.lesson.title') + ' ' + (lessonIndex + 1);
    partIndex = undefined;
    exercise = undefined;
    done = false;
  }
  loadCourse();

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

<Header {title} />

<!-- TODO: translated texts -->
<div class="description">
  Kick, switch, kick, switch. Stop stalling, let's get moving!
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

<!-- TODO: translated texts -->
<div class="overview">
  <div>Foot steps</div>
  <div>TODO</div>
  <div>Duration</div>
  <div>TODO</div>
</div>

<div class="controls">
  <a href="./record">
    <button>{$t('courses.lesson.start-button')}</button>
  </a>
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
