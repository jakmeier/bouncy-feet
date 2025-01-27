<script>
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import Step from '../../routes/collection/Step.svelte';

  /** @type {StepWrapper} */
  export let step;
  export let size = 100;
  $: lineWidth = size * 0.05;

  const beat = beatCounter;
</script>

<Step
  {size}
  {lineWidth}
  {step}
  poseIndex={$beat}
  animationTime={$timeBetweenMoves * 0.7}
/>
