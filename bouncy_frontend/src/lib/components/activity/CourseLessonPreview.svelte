<script>
  import { t } from '$lib/i18n';
  import { getContext, onDestroy, onMount } from 'svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { base } from '$app/paths';
  import Popup from '$lib/components/ui/Popup.svelte';
  import { writable } from 'svelte/store';
  import { bpm } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import { Course } from '$lib/instructor/bouncy_instructor';
  import LogoHeader from '../ui/LogoHeader.svelte';
  import TrackerPreview from '../avatar/TrackerPreview.svelte';
  import Background from '../ui/sections/Background.svelte';
  import PreviewDetails from './PreviewDetails.svelte';
  import MusicVolumeControl from '../audio/MusicVolumeControl.svelte';

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
  let size = 300;

  const songList = songs.list();
  let trackIndex = $state(0);
  /** @param {number} index */
  function changeTrack(index) {
    const track = songList[index % songList.length];
    if (track) {
      setTrack(track.id);
    }
  }

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
  <TrackerPreview {tracker} {size} />
</div>

<PreviewDetails
  durationMs={tracker.duration()}
  beats={tracker.trackedSubbeats / 2}
  {bpm}
/>

<div class="controls">
  <MusicVolumeControl />
  <button onclick={onDone}>{$t('courses.lesson.start-button')}</button>
</div>

<DarkSection>
  <h3>{$t('courses.lesson.settings-subtitle')}</h3>
  <div class="about-lesson">
    <div class="row">
      <div class="song">
        <div>{$songTitle} {$t('music.by')} {$songAuthor}</div>
        <div class="speed">
          <div>{$bpm} BPM</div>
        </div>
      </div>
      <button
        class="action right"
        onclick={() => {
          trackIndex += 1;
          changeTrack(trackIndex);
        }}>{$t('courses.lesson.next-song-button')}</button
      >
    </div>
    {#if lesson.video && lesson.video.length > 0}
      <div class="row">
        <div>{$t('record.preview-video-title')}</div>

        <button class="action big-col" onclick={() => ($isVideoOpen = true)}
          >{$t('courses.lesson.to-video-button')}</button
        >
      </div>
    {/if}
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
    margin: -0.5rem 0;
  }

  .preview {
    display: flex;
    justify-content: space-around;
    /* margin: -2rem; */
  }

  .controls {
    margin: 2rem auto;
  }
  .controls button {
    margin: 2rem 0 1rem;
  }

  .about-lesson,
  .description {
    margin: 2rem 0rem;
  }

  .about-lesson .row {
    display: flex;
    /* margin: 2rem 0; */
    gap: 1rem;
    align-items: center;
    justify-content: space-between;
  }

  .row {
    display: flex;
    flex-wrap: nowrap;
    margin: 1rem 0;
  }

  .video-wrapper {
    width: 100vw;
  }
</style>
