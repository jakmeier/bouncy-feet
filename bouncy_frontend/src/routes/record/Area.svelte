<script>
  export let width = '250px';
  export let height = '250px';
</script>

<div class="area" style="height: {height}; width: {width};">
  <slot />
</div>

<style>
  .area {
    display: grid;
    align-items: center;
    justify-items: center;
    margin: 5px auto;
    border: 5px var(--theme-neutral-dark) solid;
    border-radius: 100px;
    overflow: hidden;
  }
</style>
