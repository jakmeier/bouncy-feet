<script>
  import NumberSlider from './NumberSlider.svelte';

  export let value = 0;
  export let min = -360;
  export let max = 360;
  export let name = '';

  /** @param {number} angle */
  export let onChange = (angle) => {};
</script>

<NumberSlider {name} {min} {max} bind:value bind:onChange></NumberSlider>
