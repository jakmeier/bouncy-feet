<script>
  import Area from '../record/Area.svelte';
  import SvgAvatar from '$lib/avatar/SvgAvatar.svelte';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
  export let poseIndex = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;
  $: skeleton = step.skeleton(poseIndex);
</script>

<div>
  <Area width="{100}px" height="{100}px">
    <svg viewBox="0 0 100 100">
      <SvgAvatar
        width={100}
        height={100}
        {skeleton}
        lineWidth={4}
        leftColor="var(--theme-accent)"
        rightColor="var(--theme-main)"
        headColor="var(--theme-neutral-dark)"
        {animationTime}
      />
    </svg>
  </Area>
  <!-- TODO: translations -->
  <h3>{step.name}</h3>
</div>
