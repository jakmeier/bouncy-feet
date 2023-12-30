<script>
  import Area from '../record/Area.svelte';
  import SvgAvatar from '$lib/avatar/SvgAvatar.svelte';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
  export let size = 100;
  export let poseIndex = 0;
  /** @type{number} animationTime in ms */
  export let animationTime = 500;
  $: skeleton = step.skeleton(poseIndex);
</script>

<Area width="{size}px" height="{size}px">
  <svg viewBox="0 0 {size} {size}">
    <SvgAvatar
      width={size}
      height={size}
      {skeleton}
      lineWidth={4}
      leftColor="var(--theme-accent)"
      rightColor="var(--theme-main)"
      headColor="var(--theme-neutral-dark)"
      {animationTime}
    />
  </svg>
</Area>
