<script>
  import { beatDuration, beatStart } from '$lib/stores/Beat';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {string} text
   */

  /** @type Props */
  let { text } = $props();
  let speedSeconds = $derived($beatDuration / 500);
  /** @type {HTMLDivElement} */
  let element;

  onMount(() => {
    // Synchronize animation with the global beat
    element.getAnimations().forEach((anim) => {
      anim.startTime = $beatStart;
    });
  });
</script>

<div class="outer">
  <div class="pump" style="--pump-speed: {speedSeconds}s" bind:this={element}>
    {text}
  </div>
</div>

<style>
  .outer {
    position: absolute;
    top: 2rem;
    left: 0;
    right: 0;
    margin-inline: auto;
    width: fit-content;
  }

  .pump {
    --pump-scale: 1.2;
    --rotation: 10deg;
    text-align: center;
  }

  @keyframes pump {
    0% {
      transform: rotate(var(--rotation)) translateY(0) scale(1);
      opacity: 1;
    }
    50% {
      transform: rotate(var(--rotation)) translateY(-2px)
        scale(var(--pump-scale));
      opacity: 1;
    }
    100% {
      transform: rotate(var(--rotation)) translateY(0) scale(1);
      opacity: 1;
    }
  }

  .pump {
    color: var(--theme-neutral-darker-gray);
    transform-origin: 50% 50%;
    animation: pump var(--pump-speed) ease-in-out infinite;
    display: inline-block;
    max-width: 50vw;
  }

  /* small hover oomph (for mouse users) */
  .pump:hover {
    animation-duration: calc(var(--pump-speed) / 1.8);
    --pump-scale: 1.12;
  }

  /* Respect user preferences for reduced motion */
  @media (prefers-reduced-motion: reduce) {
    .pump {
      animation: none;
      transform: none;
    }
    .pump:hover {
      transform: none;
    }
  }
</style>
