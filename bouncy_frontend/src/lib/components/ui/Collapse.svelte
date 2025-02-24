<script>
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();
    let isHidden = $state(false);
  
    function toggleHidden() {
      isHidden = !isHidden;
    }
  </script>
  
  <style>
    .container {
      min-height: 3.0rem;
      position: relative;
      overflow: hidden;
    }
  
    .toggle-button {
      position: absolute;
      top: 0.5rem;
      right: 0.5rem;
      cursor: pointer;
      width: 1.5rem;
      height: 1.5rem;
      display: flex;
      align-items: center;
      justify-content: center;
      border: none;
      background: none;
      font-size: var(--font-normal);
      transform: rotate(180deg);
      transition: transform 0.2s ease-in-out;
    }
  
    .toggle-button.hidden {
      transform: rotate(90deg);
    }
  
    .hidden-line {
      width: 100%;
      height: 2px;
      background: #999;
      margin-top: 0.5rem;
    }
  
    .hidden-content {
      display: none;
    }
  </style>
  
  <div class="container">
    <button
      class="toggle-button {isHidden ? 'hidden' : ''}"
      onclick={toggleHidden}
      aria-label="Toggle content visibility"
    >
      ▶
    </button>
  
    {#if isHidden}
      <div class="hidden-line"></div>
    {:else}
      {@render children?.()}
    {/if}
  </div>
  