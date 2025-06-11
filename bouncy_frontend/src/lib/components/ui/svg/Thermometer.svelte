<script>
  import { onMount } from 'svelte';

  let { temperature } = $props();
  let fill = $state(0);

  onMount(() => {
    fill = temperature;
  });
</script>

<svg viewBox="0 0 100 400" width="100%" height="100%">
  <defs>
    <linearGradient id="tempGradient" x1="0" y1="1" x2="0" y2="0">
      <stop offset="{100 - fill * 100}%" stop-color="var(--theme-main)" />
      <stop offset="100%" stop-color="var(--theme-accent)" />
    </linearGradient>
  </defs>

  <rect
    x="25"
    y={(1 - fill) * 325}
    width="50"
    height={fill * 325}
    fill="url(#tempGradient)"
    rx="25"
  />

  <!-- bottom fill -->
  <path
    style="fill:#a9eb66;fill-opacity:1;stroke:none;stroke-width:12.9904;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
    id="path8419"
    d="M 91.396461,350.07849 A 41.992126,42.574314 0 0 1 49.595419,392.65236 41.992126,42.574314 0 0 1 7.4139476,350.46595 41.992126,42.574314 0 0 1 48.8311,307.50814 41.992126,42.574314 0 0 1 91.389505,349.3036"
  />

  <!-- Thermometer Outline -->
  <path
    id="path234"
    style="fill:none;stroke:#1d1d1b;stroke-width:10.0452;stroke-linecap:round;stroke-linejoin:round;stroke-dasharray:none;stroke-opacity:1"
    d="M 49.694641 5.260144 C 37.371101 5.4282321 27.501102 15.52529 27.613302 27.849463 C 27.618202 27.948373 27.623668 28.04725 27.629838 28.146086 L 27.508398 311.06938 A 44.977406 44.977364 0 0 0 5.0244995 350.4091 A 44.977406 44.977364 0 0 0 50.204687 394.97692 A 44.977406 44.977364 0 0 0 94.977665 349.99983 L 94.969914 349.18127 A 44.977406 44.977364 0 0 0 72.491699 311.07404 L 72.364575 28.146086 C 72.374175 27.979435 72.38195 27.812682 72.38783 27.645858 L 72.383695 27.238647 C 72.1596 14.916133 62.01805 5.0921521 49.694641 5.260144 z "
  />
</svg>

<style>
  svg {
    display: block;
    width: 100%;
    margin: auto;
    max-height: 100vh;
  }
  rect {
    transition: height 0.75s ease-out;
  }
</style>
