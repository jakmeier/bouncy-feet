<script>
  import { setContext } from 'svelte';
  import { writable } from 'svelte/store';

  export let color = 'black';
  export let lineWidth = 1;
  export let linecap = 'round';

  const ctx = {
    color,
    lineWidth,
    linecap,
  };
  const reactiveCtx = writable(ctx);
  setContext('svg-style', reactiveCtx);

  $: reactiveCtx.set({ color, lineWidth, linecap });
</script>

<slot />
