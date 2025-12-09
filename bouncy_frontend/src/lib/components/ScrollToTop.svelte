<script>
  import { onMount } from 'svelte';

  onMount(() => {
    document.querySelector('.background')?.scrollTo(0, 0);
  });
</script>
