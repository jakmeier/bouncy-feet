<script>
  /** @type {string | undefined} */
  export let bgColor = undefined;
  /** @type {string | undefined} */
  export let color = undefined;
</script>

<section
  class="section-container"
  style="background-color: {bgColor}; color: {color}; --background-color: {bgColor}; --color: {color};"
>
  <slot />
</section>

<style>
  .section-container {
    padding: 1rem;
    margin: calc(-1rem - 5px);
  }

  @media (min-width: 730px) {
    .section-container {
      border-radius: 5px;
    }
  }
</style>
