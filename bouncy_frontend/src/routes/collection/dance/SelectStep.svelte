<script>
  import Step from '../Step.svelte';
  import UiBox from '$lib/components/ui/UiBox.svelte';

  

  
  

  
  /**
   * @typedef {Object} Props
   * @property {import('$lib/instructor/bouncy_instructor').StepWrapper[]} steps
   * @property {boolean} [show]
   * @property {number} [poseIndex] - animation
   * @property {number} [animationTime]
   * @property {(step: import('$lib/instructor/bouncy_instructor').StepWrapper) => boolean} selectedCallback
   */

  /** @type {Props} */
  let {
    steps,
    show = $bindable(false),
    poseIndex = 0,
    animationTime = 500,
    selectedCallback
  } = $props();

  /**
   * @param {import("$lib/instructor/bouncy_instructor").StepWrapper} step
   */
  function select(step) {
    const close = selectedCallback(step);
    if (close) {
      show = false;
    }
  }
</script>

{#if show}
  <UiBox title="editor.pick-step-prompt">
    <div>
      {#each steps as step}
        <div
          onclick={() => select(step)}
          role="button"
          tabindex={0}
          onkeydown={(event) => {
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
  </UiBox>
{/if}
