<script>
  /**
   * @typedef {Object} Props
   * @property {boolean} [isOn]
   * @property {boolean} [border]
   * @property {string} [name]
   * @property {string} [ariaLabel]
   * @property {boolean} [disabled]
   * @property {(newVal: boolean)=>void} [onInput]
   */

  /** @type {Props} */
  let {
    isOn = $bindable(false),
    border = false,
    name,
    ariaLabel,
    disabled = false,
    onInput = () => {},
  } = $props();

  /** @param {{ key: string; preventDefault: () => void; }} event */
  function handleKeydown(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      toggle();
    }
  }

  function toggle() {
    if (disabled) return;
    isOn = !isOn;
    onInput(isOn);
  }
</script>

<div
  class="toggle {isOn ? 'on' : ''} {border ? 'border' : ''}"
  onclick={toggle}
  onkeydown={handleKeydown}
  role="switch"
  aria-checked={isOn}
  aria-label={ariaLabel}
  aria-disabled={disabled}
  class:disabled
  tabindex={disabled ? -1 : 0}
  id={name}
>
  <div class="toggle-circle"></div>
</div>

<style>
  .toggle {
    position: relative;
    width: 80px;
    height: 40px;
    flex-shrink: 0;
    background-color: var(--theme-neutral-dark);
    border-radius: 40px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  .toggle.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toggle:focus-visible {
    outline: 3px solid var(--theme-accent);
    outline-offset: 3px;
  }

  .toggle.on {
    background-color: var(--theme-main);
  }

  .toggle-circle {
    position: absolute;
    top: 4px;
    left: 4px;
    width: 32px;
    height: 32px;
    background-color: white;
    border-radius: 50%;
    transition: transform 0.3s;
  }

  .toggle.on .toggle-circle {
    transform: translateX(40px);
  }

  .toggle.border {
    border: solid 1px var(--theme-main);
  }
</style>
