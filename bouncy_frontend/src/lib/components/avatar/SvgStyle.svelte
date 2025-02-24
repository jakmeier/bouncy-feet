<script>
  import { setContext } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {string} [color]
   * @property {number} [lineWidth]
   * @property {string} [linecap]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    color = 'black',
    lineWidth = 1,
    linecap = 'round',
    children,
  } = $props();

  const ctx = {
    get color() {
      return color;
    },
    get lineWidth() {
      return lineWidth;
    },
    get linecap() {
      return linecap;
    },
  };
  setContext('svg-style', ctx);
</script>

{@render children?.()}
