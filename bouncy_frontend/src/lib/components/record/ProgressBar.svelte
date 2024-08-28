<script>
  export let progress = 0.0;
  $: percent = (1.0 - progress) * 100;
</script>

<div class="progress-container">
  <div class="progress-bar" style="transform: translateX({percent}%);"></div>
</div>

<style>
  .progress-container {
    width: 100%;
    height: 30px;
    background-color: var(--theme-neutral-light);
    border-radius: 5px;
    overflow: hidden;
    position: relative;
  }

  .progress-bar {
    background-color: var(--theme-main);
    width: 100%;
    height: 100%;
    position: absolute;
    bottom: 0;

    transition: transform 0.3s ease-in-out;
  }
</style>
