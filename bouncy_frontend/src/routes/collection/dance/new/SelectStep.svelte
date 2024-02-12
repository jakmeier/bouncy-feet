<script>
  import { t } from '$lib/i18n.js';
  import Step from '../../Step.svelte';
  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo[]} */
  export let steps;
  export let show = false;

  // animation
  export let poseIndex = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;

  /**
   * @type {(step: import('$lib/instructor/bouncy_instructor').StepInfo) => boolean}
   */
  export let selectedCallback;

  /**
   * @param {import("$lib/instructor/bouncy_instructor").StepInfo} step
   */
  function select(step) {
    const close = selectedCallback(step);
    if (close) {
      show = false;
    }
  }
</script>

{#if show}
  <div id="container" style="show">
    <p>{$t('editor.pick-step-instruction')}</p>
    <div class="step-table">
      {#each steps as step, i}
        <div
          on:click={() => select(step)}
          role="button"
          tabindex={i}
          on:keydown={(event) => {
            if (event.key === 'Enter' || event.key === ' ') {
              select(step);
            }
          }}
        >
          <Step {step} {poseIndex} {animationTime} />
          <!-- TODO: translations -->
          <h3>{step.name}</h3>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  p {
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
  }
  #container {
    text-align: center;
    background-color: var(--theme-neutral-light);
    width: 100%;
  }
</style>
