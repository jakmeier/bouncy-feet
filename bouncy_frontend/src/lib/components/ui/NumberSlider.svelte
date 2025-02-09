<script>
  export let value = 0;
  /** @type {number} */
  export let min;
  /** @type {number} */
  export let max;
  /** @type {number} */
  export let decimals = 0;
  export let name = '';

  /** @param {number} angle */
  export let onChange = (angle) => {};

  let isDragging = false;
  /** @type {HTMLDivElement} */
  let slider;

  function handleInput(event) {
    value = event.target.value;
  }

  function handleSliderChange(event) {
    if (!slider) return;
    const rect = slider.getBoundingClientRect();
    const frac = (event.clientX - rect.left) / rect.width;
    const exactNewValue = frac * (max - min) + min;
    const newValue = Number(
      Math.round(exactNewValue + 'e' + decimals) + 'e-' + decimals
    );
    value = Math.max(min, Math.min(newValue, max));
    onChange(value);
  }

  function startDragging(event) {
    isDragging = true;
    handleSliderChange(event);
    window.addEventListener('mousemove', handleDragging);
    window.addEventListener('mouseup', stopDragging);
  }

  function stopDragging() {
    isDragging = false;
    window.removeEventListener('mousemove', handleDragging);
    window.removeEventListener('mouseup', stopDragging);
  }

  function handleDragging(event) {
    if (!isDragging) return;
    handleSliderChange(event);
  }

  function handleKeydown(event) {
    if (event.key === 'ArrowRight' || event.key === 'ArrowUp') {
      value = Math.min(value + 1, max);
    } else if (event.key === 'ArrowLeft' || event.key === 'ArrowDown') {
      value = Math.max(value - 1, min);
    }
  }
</script>

<div class="slider-container">
  <div class="value-display">
    <input
      type="number"
      {name}
      bind:value
      {min}
      {max}
      on:input={handleInput}
      on:change={() => onChange(value)}
      aria-valuenow={value}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-label="Angle"
    />
  </div>
  <div
    bind:this={slider}
    class="slider"
    role="slider"
    tabindex="0"
    aria-valuenow={value}
    aria-valuemin={min}
    aria-valuemax={max}
    aria-label="Angle slider"
    on:click={handleSliderChange}
    on:mousedown={startDragging}
    on:keydown={handleKeydown}
  >
    <div
      class="slider-thumb"
      style="left: calc({((value - min) / (max - min)) * 100}% - 10px)"
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
  .slider {
    position: relative;
    height: 8px;
    background-color: var(--theme-neutral-white);
    border-radius: 4px;
    margin-top: 10px;
    outline: none;
  }
  .slider-thumb {
    position: absolute;
    top: calc(-50% - 2px);
    width: 20px;
    height: 20px;
    background-color: var(--theme-main);
    border-radius: 50%;
    cursor: pointer;
  }
</style>
