<script>
  // An avatar component for showing the next step to perform.
  // This includes coloring and animations specific to the instructor mode
  // where users look at the video feed and position themselves as shown
  // by the instructor stick figure.
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '../avatar/Svg.svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton} */
  export let nextSkeleton;
  export let nextBodyShift;

  const avatarLineWidth = 5;
</script>

<Svg {width} {height}>
  <SvgAvatar
    skeleton={nextSkeleton}
    {width}
    {height}
    leftColor={'#e97516C0'}
    rightColor={'#e97516C0'}
    headColor={'#ffad6960'}
    bodyColor={'#ffad6940'}
    lineWidth={avatarLineWidth}
    bodyShift={nextBodyShift}
  ></SvgAvatar>
</Svg>
