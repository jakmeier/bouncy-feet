<script>
  // A interactive wrapper for something that should be clicked but is not
  // styled like a <button> element.

  /**
   * @typedef {Object} Props
   * @property {() => void} onClick
   * @property {string} [ariaLabel]
   * @property {import('svelte').Snippet} [children]
   */

  // TODO: Make ariaLabel mandatory
  /** @type {Props} */
  let { children, onClick, ariaLabel } = $props();

  /**
   * @param {KeyboardEvent} event
   */
  function onKey(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onClick();
    }
  }
</script>

<div
  class="action"
  onclick={onClick}
  onkeydown={onKey}
  role="button"
  tabindex="0"
  aria-label={ariaLabel}
>
  {@render children?.()}
</div>

<style>
  div {
    display: contents;
  }
</style>
