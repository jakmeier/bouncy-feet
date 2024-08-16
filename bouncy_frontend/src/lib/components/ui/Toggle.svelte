<script>
  export let isOn = false;
  export let border = false;

  /** @param {{ key: string; preventDefault: () => void; }} event */
  function handleKeydown(event) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      isOn = !isOn;
    }
  }
</script>

<div
  class="toggle {isOn ? 'on' : ''} {border ? 'border' : ''}"
  on:click={() => (isOn = !isOn)}
  on:keydown={handleKeydown}
  role="switch"
  aria-checked={isOn}
  tabindex="0"
>
  <div class="toggle-circle"></div>
</div>

<style>
  .toggle {
    position: relative;
    width: 80px;
    height: 40px;
    background-color: var(--theme-neutral-light);
    border-radius: 40px;
    cursor: pointer;
    transition: background-color 0.3s;
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
