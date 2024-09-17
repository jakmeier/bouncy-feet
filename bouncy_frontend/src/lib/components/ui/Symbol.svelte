<script>
  /** @type {number|undefined} */
  export let size = undefined;

  let styleClass = $$props.class || '';
</script>

<span
  class={'material-symbols-outlined ' + styleClass}
  translate="no"
  style={size ? `font-size: ${size}px` : ''}
>
  <slot />
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
