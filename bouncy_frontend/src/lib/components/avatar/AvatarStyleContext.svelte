<script>
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { getContext, setContext } from 'svelte';

  const parentCtx = getContext('avatarStyle');

  /**
   * @typedef {Object} Props
   * @property {AvatarColoring} [coloring]
   * @property {AvatarHeadStyle} [headStyle]
   * @property {AvatarBodyShape} [bodyShape]
   * @property {PageColoring} [pageColoring]
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
    pageColoring = parentCtx
      ? parentCtx.danceFloorColor
      : {
          pageColor: 'var(--theme-neutral-white)',
          fontColor: 'var(--theme-neutral-black)',
          danceFloorColor: 'var(--theme-neutral-white)',
        },
    children,
  } = $props();

  /** @type {AvatarStyleContext} */
  let ctx = $state({
    get coloring() {
      return coloring;
    },
    get headStyle() {
      return headStyle;
    },
    get bodyShape() {
      return bodyShape;
    },
    get pageColoring() {
      return pageColoring;
    },
  });
  setContext('avatarStyle', ctx);
</script>

{@render children?.()}
