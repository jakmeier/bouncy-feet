<script>
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import Step from '../../routes/collection/Step.svelte';

  /** @type {StepWrapper} */
  export let step;
  export let size = 100;
  export let backgroundColor = 'var(--theme-neutral-white)';

  const beat = beatCounter;
</script>

<Step
  {size}
  {step}
  poseIndex={$beat}
  animationTime={$timeBetweenMoves * 0.7}
  {backgroundColor}
/>
