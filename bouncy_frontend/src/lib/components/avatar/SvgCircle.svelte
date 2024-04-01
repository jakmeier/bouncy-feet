<script>
  import { getContext } from 'svelte';

  /* @type{number} */
  export let cx;
  /* @type{number} */
  export let cy;
  /* @type{number} */
  export let r;
  /* @type{string} */
  export let fill = 'black';
  /** @type {string} */
  export let id;

  const svg = getContext('svg');
  $: svg.setCircle(id, { cx, cy, r, fill });
</script>
