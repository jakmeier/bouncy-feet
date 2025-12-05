<script>
  /** @typedef {object} Props
   * @prop {number} [size]
   * @prop {string} [styleClass]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props}
   */
  let { size, styleClass = '', children } = $props();
</script>

<span
  class={'material-symbols-outlined ' + styleClass}
  translate="no"
  style={size ? `font-size: ${size}px` : ''}
>
  {@render children?.()}
</span>

<style>
  .thin {
    line-height: 30px;
  }

  .white {
    color: var(--theme-neutral-white);
  }

  .rotating {
    animation: rotate 1.5s linear infinite;
  }
  @keyframes rotate {
    to {
      transform: rotate(360deg);
    }
  }
</style>
