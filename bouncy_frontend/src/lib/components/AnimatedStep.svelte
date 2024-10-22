<script>
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import Step from '../../routes/collection/Step.svelte';

  /** @type {StepWrapper} */
  export let step;
  export let size = 100;

  const beat = beatCounter;
  $: step,
    console.log(
      'step updated',
      step.jumpHeight(0),
      step.jumpHeight(1),
      step.jumpHeight(2)
    );
</script>

<Step {size} {step} poseIndex={$beat} animationTime={$timeBetweenMoves * 0.7} />
