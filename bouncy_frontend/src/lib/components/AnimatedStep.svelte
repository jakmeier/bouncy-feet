<script>
  import { StepWrapper } from '$lib/instructor/bouncy_instructor';
  import { beatCounter, timeBetweenMoves } from '$lib/stores/Beat';
  import Step from '../../routes/collection/Step.svelte';

  /**
   * @typedef {Object} Props
   * @property {StepWrapper} step
   * @property {number} [size]
   * @property {string} [backgroundColor]
   * @property {string} [style]
   */

  /** @type {Props} */
  let {
    step,
    size = 100,
    backgroundColor = 'var(--theme-neutral-white)',
    style = 'rounded',
  } = $props();

  const beat = beatCounter;
</script>

<Step
  {size}
  {step}
  poseIndex={$beat}
  animationTime={$timeBetweenMoves * 0.7}
  {backgroundColor}
  {style}
/>
