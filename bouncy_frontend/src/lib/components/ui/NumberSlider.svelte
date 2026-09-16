<script>
  /**
   * @typedef {Object} Props
   * @property {number} [value]
   * @property {number} min
   * @property {number} max
   * @property {number} [decimals]
   * @property {string} [name]
   * @property {(value: number) => void} [onChange]
   * @property {string} [thumbColor]
   * @property {string} [backgroundColor]
   * @property {string} [unitLabel]
   * @property {string} [ariaLabel]
   */

  /** @type {Props} */
  let {
    value = $bindable(0),
    min,
    max,
    decimals = 0,
    name = '',
    onChange = () => {},
    thumbColor = 'var(--theme-main)',
    backgroundColor = 'var(--theme-neutral-white)',
    unitLabel,
    ariaLabel = 'Slider',
  } = $props();

  let isDragging = false;
  /** @type {HTMLDivElement | null} */
  let slider = $state(null);
  /** @type {HTMLDivElement | null} */
  let thumb = $state(null);

  /** @param {Event} event */
  function handleInput(event) {
    value = Number(/** @type {HTMLInputElement} */ (event.currentTarget).value);
  }

  /** @param {{clientX: number}} event */
  function handleSliderChange(event) {
    if (!slider) return;
    const rect = slider.getBoundingClientRect();
    const frac = (event.clientX - rect.left) / rect.width;
    const exactNewValue = frac * (max - min) + min;
    const decimalFactor = 10 ** decimals;
    const newValue = Math.round(exactNewValue * decimalFactor) / decimalFactor;
    value = Math.max(min, Math.min(newValue, max));
    onChange(value);
  }

  /** @param {PointerEvent} event */
  function startDragging(event) {
    isDragging = true;
    const target = /** @type {HTMLElement} */ (event.currentTarget);
    target.setPointerCapture(event.pointerId);
    handleSliderChange(event);
  }

  /** @param {PointerEvent} event */
  function stopDragging(event) {
    isDragging = false;
    const target = /** @type {HTMLElement} */ (event.currentTarget);
    if (target.hasPointerCapture(event.pointerId)) {
      target.releasePointerCapture(event.pointerId);
    }
  }

  /** @param {PointerEvent} event */
  function handleDragging(event) {
    if (!isDragging) return;
    handleSliderChange(event);
  }

  /** @param {KeyboardEvent} event */
  function handleKeydown(event) {
    if (event.key.startsWith('Arrow')) event.preventDefault();
    if (event.key === 'ArrowRight' || event.key === 'ArrowUp') {
      value = Math.min(value + 1, max);
      onChange(value);
    } else if (event.key === 'ArrowLeft' || event.key === 'ArrowDown') {
      value = Math.max(value - 1, min);
      onChange(value);
    }
  }

  // Requires some juggling to get a computed color I can put inside an inlined SVG
  let hatchColor = $derived.by(() => {
    if (!thumb) {
      return 'white';
    }
    const computed = getComputedStyle(thumb);
    return computed.backgroundColor;
  });

  let hatchBackground = $derived(
    encodeURIComponent(`
      <svg xmlns="http://www.w3.org/2000/svg" width="8" height="8">
        <line x1="0" y1="8" x2="8" y2="0" stroke="${hatchColor}" stroke-width="3" stroke-linecap="round"/>
      </svg>`)
  );
</script>

<div class="slider-container">
  <div class="value-display">
    {#if unitLabel}
      {value} {unitLabel}
    {:else}
      <input
        type="number"
        {name}
        bind:value
        {min}
        {max}
        oninput={handleInput}
        onchange={() => onChange(value)}
        aria-valuenow={value}
        aria-valuemin={min}
        aria-valuemax={max}
        aria-label={ariaLabel}
      />
    {/if}
  </div>
  <div
    bind:this={slider}
    class="slider"
    role="slider"
    tabindex="0"
    aria-valuenow={value}
    aria-valuemin={min}
    aria-valuemax={max}
    aria-label={ariaLabel}
    onpointerdown={startDragging}
    onpointermove={handleDragging}
    onpointerup={stopDragging}
    onpointercancel={stopDragging}
    onkeydown={handleKeydown}
    style="background-color: {backgroundColor}"
  >
    <div
      class="slider-filled"
      style="width: calc({((value - min) / (max - min)) * 100}% ); 
        background-image: url('data:image/svg+xml,{hatchBackground}');
        "
      aria-hidden="true"
    ></div>
    <div
      bind:this={thumb}
      class="slider-thumb"
      style="left: calc({((value - min) / (max - min)) *
        100}% - 10px); background-color: {thumbColor};"
      aria-hidden="true"
    ></div>
  </div>
</div>

<style>
  .slider-container {
    width: 90%;
    margin: auto;
    text-align: center;
  }
  .value-display input {
    border: none;
    text-align: center;
    font-size: var(--font-normal);
    background: none;
    width: 100%;
  }
  .slider-filled,
  .slider {
    position: relative;
    height: 8px;
    border-radius: 4px;
    margin-top: 10px;
    outline: none;
    touch-action: none;
  }
  .slider:focus-visible {
    outline: 3px solid var(--theme-accent);
    outline-offset: 3px;
  }
  .slider-thumb {
    position: absolute;
    top: calc(-50% - 2px);
    width: 20px;
    height: 20px;
    border-radius: 50%;
    cursor: pointer;
  }

  .slider-filled {
    height: 8px;
    background-repeat: repeat;
  }
</style>
