<script>
  import { run } from 'svelte/legacy';

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
   * @property {number} [dummyUpdate] - To trigger reactivity even when values are the same. Needed for jumps.
   */

  /** @type {Props} */
  let {
    cx,
    cy,
    r,
    fill,
    stroke,
    strokeWidth,
    id,
    dummyUpdate = 0
  } = $props();

  const svg = getContext('svg');
  run(() => {
    dummyUpdate,
      svg.setCircle(id, { cx, cy, r, fill, dummyUpdate, strokeWidth, stroke });
  });

  onMount(() => svg.setCircle(id, { cx, cy, r, fill, strokeWidth, stroke }));
  onDestroy(() => svg.removeCircle(id));
</script>
