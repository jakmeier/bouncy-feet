<script>
  export let width = '250px';
  export let height = '250px';
  export let borderRadius = '100px';
  export let borderWidth = '5px';
</script>

<div
  class="area"
  style="height: {height}; width: {width}; border-radius: {borderRadius}; border: {borderWidth} var(--theme-neutral-dark) solid;"
>
  <slot />
</div>

<style>
  .area {
    display: grid;
    align-items: center;
    justify-items: center;
    margin: 5px auto;
    overflow: hidden;
  }
</style>
