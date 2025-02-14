<script>
  //! Show content on the full available screen, or, if it would exceed that
  //! maximum configured area, on a limited area on top of the other content.
  //! Usually this means, full-screen on mobile only.

  export let maxWidth = 600;
  export let maxHeight = 900;
  export let backgroundColor = 'none';
  export let overlayColor = 'none';
</script>

<div class="overlay" style="background-color: {overlayColor}">
  <div
    class="area"
    style="height: min(100vh, {maxHeight}px); width: min(100vw, {maxWidth}px); background-color: {backgroundColor};"
  >
    <slot />
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }
  .area {
    position: relative;
    display: grid;
    align-items: center;
    justify-items: center;
    margin: auto;
    overflow: hidden;
  }
</style>
