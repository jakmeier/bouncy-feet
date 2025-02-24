<script>
  import { getContext, onDestroy, onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {any} cx
   * @property {any} cy
   * @property {any} r
   * @property {any} fill
   * @property {any} stroke
   * @property {any} strokeWidth
   * @property {string} id
   */

  /** @type {Props} */
  let { cx, cy, r, fill, stroke, strokeWidth, id } = $props();

  const svg = getContext('svg');
  $effect(() => {
    svg.setCircle(id, {
      cx,
      cy,
      r,
      fill,
      strokeWidth,
      stroke,
    });
  });

  onMount(() => svg.setCircle(id, { cx, cy, r, fill, strokeWidth, stroke }));
  onDestroy(() => svg.removeCircle(id));
</script>
