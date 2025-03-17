<script>
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import { UGLY_LEFT_RIGHT_COLORING } from '$lib/constants';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();
</script>

<AvatarStyleContext
  coloring={UGLY_LEFT_RIGHT_COLORING}
  bodyShape={{ strokeWidth: 1.0 }}
>
  {@render children?.()}
</AvatarStyleContext>
