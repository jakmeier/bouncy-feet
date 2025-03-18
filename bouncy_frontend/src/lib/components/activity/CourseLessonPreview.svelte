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
  import ActivityPreview from './ActivityPreview.svelte';

  /**
   * @typedef {Object} Props
   * @property {Course} course
   * @property {number} lessonIndex
   * @property {function} onDone
   */

  /** @type {Props}*/
  let { course, lessonIndex, onDone } = $props();

  const { tracker } = getContext('tracker');

  let lesson = course.lessons[lessonIndex];
  const video = lesson.video;
  /** @type {string} */
  let title = course.lessons[lessonIndex].name;
  let description = course.lessons[lessonIndex].explanation;

  const songList = songs.list();

  // TODO; Decide based on lesson or something
  const trackId = songList[0].id;
</script>

<ActivityPreview {title} {description} {onDone} {tracker} {video} {trackId} />
