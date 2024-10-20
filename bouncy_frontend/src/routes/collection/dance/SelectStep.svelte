<script>
  import Step from '../Step.svelte';
  import UiBox from '$lib/components/ui/UiBox.svelte';

  /** @type {import('$lib/instructor/bouncy_instructor').StepWrapper[]} */
  export let steps;
  export let show = false;

  // animation
  export let poseIndex = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;

  /**
   * @type {(step: import('$lib/instructor/bouncy_instructor').StepWrapper) => boolean}
   */
  export let selectedCallback;

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
          on:click={() => select(step)}
          role="button"
          tabindex={0}
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
  </UiBox>
{/if}
