<script>
  import Canvas from '$lib/Canvas.svelte';
  import Area from './record/Area.svelte';
  import Avatar from './record/Avatar.svelte';
  import SvgAvatar from '$lib/avatar/SvgAvatar.svelte';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
  export let poseIndex = 0;
  $: skeleton = step.skeleton(poseIndex);
</script>

<div>
  <Area width="{100}px" height="{100}px">
    <svg viewBox="0 0 100 100">
      <SvgAvatar width={100} height={100} {skeleton} lineWidth={4} />
    </svg>
  </Area>
  <!-- TODO: translations -->
  <h3>{step.name}</h3>
</div>
