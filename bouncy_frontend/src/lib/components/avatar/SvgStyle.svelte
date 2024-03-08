<script>
  import { setContext } from 'svelte';

  export let color = 'black';
  export let lineWidth = 1;
  export let linecap = 'round';

  setContext('svg-style', {
    color,
    lineWidth,
    linecap,
  });
</script>

<slot />
