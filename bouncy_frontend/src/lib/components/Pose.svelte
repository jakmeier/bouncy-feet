<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  
  /**
   * @typedef {Object} Props
   * @property {PoseWrapper} pose
   * @property {number} [size]
   */

  /** @type {Props} */
  let { pose, size = 200 } = $props();
</script>

<div class="avatar">
  <Svg width={size} height={size} orderByZ>
    <SvgAvatar skeleton={pose.skeleton()} width={size} height={size}
    ></SvgAvatar>
  </Svg>
</div>
