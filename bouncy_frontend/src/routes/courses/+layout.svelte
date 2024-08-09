<script>
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';

  /** @type {import('./$types').LayoutData} */
  export let data;
  const courses = data.courses;

  const getCourse = (/** @type {string} */ id) =>
    courses.find((course) => course.id === id);
  setContext('courses', { courses: readable(courses), getCourse });
</script>

<slot />
