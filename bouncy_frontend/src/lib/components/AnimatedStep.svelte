<script>
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { timeBetweenMoves } from '$lib/stores/Beat';
  import { counter } from '$lib/timer';
  import Step from '../../routes/collection/Step.svelte';

  /** @type {StepWrapper} */
  export let step;
  export let size = 100;

  const beat = counter(-1, 1, $timeBetweenMoves);
</script>

<Step {size} {step} poseIndex={$beat} animationTime={$timeBetweenMoves * 0.7} />
