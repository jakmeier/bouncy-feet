<script>
  import NumberSlider from './NumberSlider.svelte';


  
  /**
   * @typedef {Object} Props
   * @property {number} [value]
   * @property {any} [min]
   * @property {number} [max]
   * @property {string} [name]
   * @property {any} [onChange]
   */

  /** @type {Props} */
  let {
    value = $bindable(0),
    min = -360,
    max = 360,
    name = '',
    onChange = $bindable((angle) => {})
  } = $props();
</script>

<NumberSlider {name} {min} {max} bind:value bind:onChange></NumberSlider>
