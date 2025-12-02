<script>
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').LayoutData} data
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { data, children } = $props();
  const courses = data.courses;

  const getCourse = (/** @type {string} */ id) =>
    courses.find((course) => course.id === id);
  setContext('courses', { courses: readable(courses), getCourse });
</script>

{@render children?.()}
