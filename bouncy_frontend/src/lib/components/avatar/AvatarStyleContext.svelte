<script>
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { getContext, setContext } from 'svelte';

  const parentCtx = getContext('avatarStyle');

  

  
  
  /**
   * @typedef {Object} Props
   * @property {AvatarColoring} [coloring]
   * @property {AvatarHeadStyle} [headStyle]
   * @property {AvatarBodyShape} [bodyShape]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    coloring = parentCtx ? parentCtx.coloring : LEFT_RIGHT_COLORING,
    headStyle = parentCtx
    ? parentCtx.headStyle
    : {
        shape: 'disk',
        size: 1,
        strokeWidth: 1,
      },
    bodyShape = parentCtx
    ? parentCtx.bodyShape
    : {
        // height: 1,
        // width: 1,
        strokeWidth: 1,
      },
    children
  } = $props();

  /** @type {AvatarStyleContext} */
  let ctx = {
    coloring,
    headStyle,
    bodyShape,
  };
  setContext('avatarStyle', ctx);
</script>

{@render children?.()}
