<script>
  import { StepPositionBuilder } from '$lib/instructor/bouncy_instructor';

  /** @type {StepPositionBuilder} */
  export let position;

  $: pose = position.pose();
</script>

<h2>{pose.name('en')}</h2>
{position.jumpHeight}

