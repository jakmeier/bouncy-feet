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

  // TODO; Decide based on lesson or something
  const trackId = songList[0].id;
</script>

<ActivityPreview
  {title}
  {description}
  {onDone}
  {tracker}
  {video}
  {trackId}
  {difficulty}
  {energy}
  {onBack}
/>
