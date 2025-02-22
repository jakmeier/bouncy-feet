<script>
  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { setContext } from 'svelte';

  /** @type {AvatarColoring} */
  export let coloring = LEFT_RIGHT_COLORING;

  /** @type {AvatarHeadStyle} */
  export let headStyle = {
    shape: 'disk',
    size: 1,
    strokeWidth: 1,
  };
  /** @type {AvatarBodyShape} */
  export let bodyShape = {
    // height: 1,
    // width: 1,
    strokeWidth: 1,
  };

  /** @type {AvatarStyleContext} */
  let ctx = {
    coloring,
    headStyle,
    bodyShape,
  };
  setContext('avatarStyle', ctx);
</script>

<slot />
