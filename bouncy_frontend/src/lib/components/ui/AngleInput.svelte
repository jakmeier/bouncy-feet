<script>
  import NumberSlider from './NumberSlider.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} [value]
   * @property {any} [min]
   * @property {number} [max]
   * @property {string} [name]
   * @property {any} [onChange]
   * @property {string} [thumbColor]
   */

  /** @type {Props} */
  let {
    value = $bindable(0),
    min = -360,
    max = 360,
    name = '',
    thumbColor,
    onChange = $bindable((_angle) => {}),
  } = $props();
</script>

<NumberSlider {name} {min} {max} {thumbColor} bind:value {onChange}
></NumberSlider>
