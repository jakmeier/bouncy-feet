<script>
  import { getContext } from 'svelte';
  import AvatarStyleContext from './AvatarStyleContext.svelte';

  /**
   * Read the avatar style from the user, using parent context values.
   * Then set it as the AvatarStyleContext for children.
   */

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  /** @type {LocalState} */
  const localState = getContext('localState');
</script>

<AvatarStyleContext
  coloring={localState.avatarStyle.coloring}
  headStyle={localState.avatarStyle.headStyle}
  bodyShape={localState.avatarStyle.bodyShape}
>
  {@render children?.()}
</AvatarStyleContext>
