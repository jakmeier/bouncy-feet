<script>
  /**
   * @type {any[]}
   */
  let movingObjects = [];

  const stripeConfigs = [
    { color: 'green', count: 8 },
    { color: 'dark-green', count: 2 },
    { color: 'orange', count: 2 },
  ];

  function randomStripe({ color, reverse }) {
    const height = `${Math.random() * 8 + 1}vh`;
    const top = `${Math.random() * 200 - 50}vh`;
    const duration = Math.random() * 30 + 15;
    const startRotation = Math.random() * 80 - 45;
    const endRotation = startRotation + Math.random() * 120 - 60;

    return {
      color,
      reverse,
      height,
      top,
      duration,
      startRotation,
      endRotation,
    };
  }

  function addDynamicStripes() {
    stripeConfigs.forEach(({ color, count }) => {
      for (let i = 0; i < count; i++) {
        const reverse = (i & 2) === 0;
        const stripe = randomStripe({ color, reverse });
        movingObjects.push(stripe);
      }
    });
  }

  addDynamicStripes();
  //   onMount(addDynamicStripes);
</script>

<div class="container">
  <div class="moving-objects">
    {#each movingObjects as obj}
      <div
        class="{obj.color} stripe move"
        class:reverse={obj.reverse}
        style="min-height: {obj.height}; top: {obj.top}; animation-duration: {obj.duration}s; --start-rotation: {obj.startRotation}deg; --end-rotation: {obj.endRotation}deg;"
      ></div>
    {/each}
  </div>

  <!-- Slot for your content -->
  <div class="content">
    <slot />
  </div>
</div>

<style>
  .container {
    position: relative;
    /* 5px padding */
    width: calc(100% - 10px);
    min-height: 100vh;
    /* overflow: hidden; */
  }

  .moving-objects {
    position: fixed;
    width: 100%;
    height: 100%;
    inset: 0;
    z-index: -1;
    overflow: hidden;
    filter: blur(4px);
  }

  .stripe {
    position: absolute;
    width: 200%;
    left: -50%;
    min-height: 10vh;
    opacity: 0.76;
  }

  .green {
    background-color: var(--theme-main);
  }

  .dark-green {
    background-color: var(--theme-main-dark);
  }

  .orange {
    background-color: var(--theme-accent);
  }

  .move {
    /* animation: move 9s linear 0s infinite alternate; */
    /* instead of moving, just fix it? to the start? */
    transform: translateY(-50vh) rotate(var(--start-rotation, 0deg));
  }
  .reverse {
    animation-direction: alternate-reverse;
  }

  @keyframes move {
    from {
      transform: translateY(-50vh) rotate(var(--start-rotation, 0deg));
    }
    to {
      transform: translateY(50vh) rotate(var(--end-rotation, 20deg));
    }
  }
  .content {
    z-index: 1;
    max-width: 100%;
    margin: 0 auto;
  }
</style>
