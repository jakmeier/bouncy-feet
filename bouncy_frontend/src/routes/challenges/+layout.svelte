<script>
  // @ts-nocheck
  import { setContext } from 'svelte';
  import { readable } from 'svelte/store';
  let { data, children } = $props();
  const sets = data.challengeSets;
  setContext('challengeSets', {
    sets: readable(sets),
    getSet: (id) => sets.find((set) => set.id === id),
  });
</script>

{@render children?.()}
