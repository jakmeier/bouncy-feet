<script>
  import { onDestroy, onMount } from 'svelte';
  import { backgroundColor, fontColor } from '$lib/stores/UiState';

  
  
  /**
   * @typedef {Object} Props
   * @property {string | undefined} [bgColor]
   * @property {string | undefined} [color]
   */

  /** @type {Props} */
  let { bgColor = undefined, color = undefined } = $props();

  /** @type {string | undefined} */
  let swapBackgroundColor = undefined;
  /** @type {string | undefined} */
  let swapFontColor = undefined;

  onMount(() => {
    if (bgColor) {
      swapBackgroundColor = $backgroundColor;
      $backgroundColor = bgColor;
    }
    if (color) {
      swapFontColor = $fontColor;
      $fontColor = color;
    }
  });
  onDestroy(() => {
    if (swapBackgroundColor) {
      $backgroundColor = swapBackgroundColor;
    }
    if (swapFontColor) {
      $fontColor = swapFontColor;
    }
  });
</script>
