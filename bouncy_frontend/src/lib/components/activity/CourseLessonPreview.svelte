<script>
  import { getContext } from 'svelte';
  import { songs } from '$lib/stores/Songs';
  import { Course } from '$lib/instructor/bouncy_instructor';
  import ActivityPreview from './ActivityPreview.svelte';

  /**
   * @typedef {Object} Props
   * @property {Course} course
   * @property {number} lessonIndex
   * @property {function} onDone
   * @property {()=>void} onBack
   */

  /** @type {Props}*/
  let { course, lessonIndex, onDone, onBack } = $props();

  const { tracker } = getContext('tracker');

  let lesson = course.lessons[lessonIndex];
  const video = lesson.video;
  /** @type {string} */
  let title = course.lessons[lessonIndex].name;
  let description = course.lessons[lessonIndex].explanation;
  const difficulty = course.lessons[lessonIndex].difficulty;
  const energy = course.lessons[lessonIndex].energy;

  const songList = songs.list();

  const trackId = course.lessons[lessonIndex].song || songList[0].id;
  const trackTimestamp = course.lessons[lessonIndex].songTimestamp;
</script>

<ActivityPreview
  {title}
  {description}
  {onDone}
  {tracker}
  {video}
  {trackId}
  {trackTimestamp}
  {difficulty}
  {energy}
  {onBack}
/>
