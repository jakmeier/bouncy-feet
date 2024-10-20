<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  /** @type {PoseWrapper} */
  export let pose;
  export let size = 200;
  export let style = LEFT_RIGHT_COLORING_LIGHT;
</script>

<div class="avatar">
  <Svg width={size} height={size} orderByZ>
    <SvgAvatar skeleton={pose.skeleton()} width={size} height={size} {style}
    ></SvgAvatar>
  </Svg>
</div>
