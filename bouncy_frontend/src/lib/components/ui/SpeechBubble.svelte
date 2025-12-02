<script>
  import { createParagraphs } from '$lib/text_utils';

  /**
   * @typedef {Object} Props
   * @property {string} text
   * @property {string} [backgroundColor]
   * @property {string} [textColor]
   * @property {string} [position]
   * @property {string} [right]
   * @property {string} [width]
   * @property {number} [tailSize]
   */

  /** @type {Props} */
  let {
    text,
    backgroundColor = 'var(--theme-neutral-light)',
    textColor = 'var(--theme-neutral-dark)',
    position = 'top',
    right = '50%',
    width = '300px',
    tailSize = 10,
  } = $props();

  let textLines = $derived(createParagraphs(text));
</script>

<div
  class="speech-bubble"
  style="--background-color: {backgroundColor}; --text-color: {textColor}; --tail-size: {tailSize}px; max-width: {width}"
>
  {#each textLines as line}
    <p>{line}</p>
  {/each}
  <div class="tail {position}" style="right: {right}"></div>
</div>

<style>
  .speech-bubble {
    position: relative;
    margin: auto;
    padding: 5px;
    background-color: var(--background-color);
    color: var(--text-color);
    border-radius: 40px;
    text-align: center;
    opacity: 1;
    animation: fade-in 0.5s linear;
  }
  @keyframes fade-in {
    0% {
      opacity: 0;
    }
    100% {
      opacity: 1;
    }
  }

  .tail {
    content: '';
    position: absolute;
    width: 0;
    height: 0;
    border-style: solid;
  }

  .top {
    top: calc(-1 * var(--tail-size));
    transform: translateX(-50%);
    border-width: 0 var(--tail-size) var(--tail-size) var(--tail-size);
    border-color: transparent transparent var(--background-color) transparent;
  }

  .bottom {
    bottom: calc(-1 * var(--tail-size));
    transform: translateX(-50%);
    border-width: var(--tail-size) var(--tail-size) 0 var(--tail-size);
    border-color: var(--background-color) transparent transparent transparent;
  }

  p {
    margin: 12px 5px;
  }
</style>
