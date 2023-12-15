<script>
  import Canvas from '$lib/Canvas.svelte';
  import Area from './record/Area.svelte';
  import Avatar from './record/Avatar.svelte';

  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;
</script>

<div>
  <Area width="{100}px" height="{100}px">
    <Canvas width={300} height={300}>
      <Avatar
        width={100}
        height={100}
        skeleton={step.skeleton(0)}
        lineWidth={4}
      />
    </Canvas>
  </Area>
  <!-- TODO: translations -->
  <h3>{step.name}</h3>
</div>
