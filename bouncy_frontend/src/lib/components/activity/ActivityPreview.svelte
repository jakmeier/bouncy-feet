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
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import LogoHeader from '../ui/LogoHeader.svelte';
  import TrackerPreview from '../avatar/TrackerPreview.svelte';
  import Background from '../ui/sections/Background.svelte';
  import PreviewDetails from './PreviewDetails.svelte';
  import MusicVolumeControl from '../audio/MusicVolumeControl.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} title
   * @property {string} description
   * @property {function} onDone
   * @property {Tracker} tracker
   * @property {string} [video]
   * @property {string} trackId
   * @property {number} difficulty
   * @property {number} energy
   * @property {()=>void} onBack
   */

  /** @type {Props}*/
  let {
    title,
    description,
    tracker,
    onDone,
    video,
    difficulty,
    energy,
    onBack,
  } = $props();

  const { stopTrack, setTrack, resumeTrack, songTitle, songAuthor } =
    getContext('music');

  let isVideoOpen = $state(writable(false));
  let fullWidth = $state();

  const songList = songs.list();
  let trackIndex = $state(0);
  /** @param {number} index */
  function changeTrack(index) {
    const track = songList[index % songList.length];
    if (track) {
      setTrack(track.id);
      resumeTrack();
    }
  }

  onMount(() => {
    setTrack(songList[0].id);
    resumeTrack();
  });
  onDestroy(() => {
    stopTrack();
  });
</script>

<Background bgColor="var(--theme-main)" color="var(--theme-neutral-black)" />

<LogoHeader mainColor {title} backButton {onBack} />

<div class="description">
  {description}
</div>

<div class="preview" bind:clientWidth={fullWidth}>
  <TrackerPreview {tracker} size={fullWidth} />
</div>

<PreviewDetails
  durationMs={tracker.duration()}
  beats={tracker.trackedSubbeats / 2}
  {bpm}
  {difficulty}
  {energy}
/>

<div class="controls">
  <MusicVolumeControl />
  <button onclick={onDone}>{$t('courses.lesson.start-button')}</button>
</div>

<DarkSection>
  <h1>{$t('courses.lesson.settings-subtitle')}</h1>
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
    {#if video && video.length > 0}
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
    {#if video && video.length > 0}
      <Video path={`${base}${video}`}></Video>
    {/if}
  </div>
</Popup>

<style>
  .preview {
    display: flex;
    justify-content: space-around;
    /* margin: -2rem; */
  }

  .controls {
    margin: 1.5rem auto 0;
  }
  .controls button {
    margin: 1.5rem 0;
  }

  .about-lesson,
  .description {
    margin-bottom: 2rem;
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
