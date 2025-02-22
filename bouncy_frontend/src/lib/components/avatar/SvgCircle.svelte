<script>
  import { getContext, onDestroy, onMount } from 'svelte';

  /* @type{number} */
  export let cx;
  /* @type{number} */
  export let cy;
  /* @type{number} */
  export let r;
  /* @type{string} */
  export let fill;
  /* @type{string} */
  export let stroke;
  /* @type{number} */
  export let strokeWidth;
  /** @type {string} */
  export let id;

  // To trigger reactivity even when values are the same. Needed for jumps.
  export let dummyUpdate = 0;

  const svg = getContext('svg');
  $: dummyUpdate,
    svg.setCircle(id, { cx, cy, r, fill, dummyUpdate, strokeWidth, stroke });

  onMount(() => svg.setCircle(id, { cx, cy, r, fill, strokeWidth, stroke }));
  onDestroy(() => svg.removeCircle(id));
</script>
