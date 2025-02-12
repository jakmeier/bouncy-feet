<script>
  /** @type {string | undefined} */
  export let bgColor = undefined;
  /** @type {string | undefined} */
  export let color = undefined;
</script>

<div
  class="section-container"
  style="background-color: {bgColor}; color: {color}; --background-color: {bgColor}; --color: {color};"
>
  <slot />
</div>

<style>
  .section-container {
    padding: 1rem;
    margin: calc(-1rem - 5px);
  }
</style>
