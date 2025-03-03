<script>
  import { base } from '$app/paths';
  import WarmUp from '$lib/components/activity/WarmUp.svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const step = data.lookupSteps({ stepName: 'Knee Up', uniqueNames: true })[0];

  // TODO: This should be defined in the course / lesson / warmup
  const description =
    'I will make some moves, you try to copy me. Can you keep up?';
</script>

<!-- TODO: real video, real step -->
<AvatarStyleContext>
  <WarmUp {step} videoUrl={`${base}`} {description} audioControl={false}
  ></WarmUp>
</AvatarStyleContext>

<!-- After: Simplified review  -->
<!-- Then: First (real) lesson: step side-to-side -->
<!-- Then: Show full review -->
<!-- Then: Second lesson: heel forward left and right -->
<!-- Then: Show full review -->

<!-- Then: Show list of active moves for repetition lesson -->
<!-- Then: First repetition lesson -->
<!-- Then: Show full review -->

<!-- Then: Show teacher selection -->
