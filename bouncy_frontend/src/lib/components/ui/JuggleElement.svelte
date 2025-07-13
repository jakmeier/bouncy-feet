<script>
  import { Spring } from 'svelte/motion';

  let { children, x } = $props();

  const spring = Spring.of(() => x);
</script>

<div class="box" style="transform: translate({spring.current}%, 0)">
  {@render children?.()}
</div>

<style>
  .box {
    position: absolute;
    top: 0;
    left: 0;
  }
</style>
