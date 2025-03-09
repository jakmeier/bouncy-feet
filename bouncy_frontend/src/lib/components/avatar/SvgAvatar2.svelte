<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import SvgStyle from './SvgStyle.svelte';
  import {
    Cartesian2d,
    RenderableSegment,
  } from 'bouncy_instructor';
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';
  import SvgAvatarHead from './SvgAvatarHead.svelte';

  /**
   * @typedef {Object} Props
   * @property {any} skeleton
   * @property {number} [avatarSizePixels]
   * @property {RenderableSegment[]} [markedSegments]
   */

  /** @type {Props} */
  let { skeleton, avatarSizePixels, markedSegments = [] } = $props();

  const markerColor = '#ff111166';
  const markerLineWidth = 16;

  /** @type {AvatarStyleContext} */
  const avatarStyleCtx = getContext('avatarStyle');

  /** @type {AvatarColoring} */
  let coloring = $derived(avatarStyleCtx.coloring);
  let lineWidth = $derived(10 * avatarStyleCtx.bodyShape.strokeWidth);
  let headHeight = 1;

  let markedSegmentsLines = $derived(
    markedSegments.map((segment, i) => {
      return {
        id: `marker${i}`,
        start: segment.start,
        end: segment.end,
        z: segment.z - 1,
        style: {
          color: markerColor,
          linecap: 'round',
          lineWidth: markerLineWidth,
        },
      };
    })
  );
  /** @type {Cartesian2d} */
  let leftHip = $derived(skeleton.hip.start);
  /** @type {Cartesian2d} */
  let leftShoulder = $derived(skeleton.shoulder.start);
  /** @type {Cartesian2d} */
  let rightHip = $derived(skeleton.hip.end);
  /** @type {Cartesian2d} */
  let rightShoulder = $derived(skeleton.shoulder.end);
  /** @type {number} */
  let cx = $derived((leftShoulder.x + rightShoulder.x) / 2);
  /** @type {number} */
  let cy = $derived(leftShoulder.y - (avatarSizePixels * headHeight) / 10);
</script>

<SvgAvatarHead
  {cx}
  {cy}
  color={coloring.headColor}
  headStyle={avatarStyleCtx.headStyle}
  {avatarSizePixels}
/>
<SvgStyle color={coloring.leftColor} linecap="round" {lineWidth}>
  <SvgAvatarSide side={skeleton.left} sideId={'left'}></SvgAvatarSide>
</SvgStyle>
<SvgStyle color={coloring.rightColor} linecap="round" {lineWidth}>
  <SvgAvatarSide side={skeleton.right} sideId={'right'}></SvgAvatarSide>
</SvgStyle>
{#each markedSegmentsLines as line}
  <SvgLine id={line.id} {line}></SvgLine>
{/each}
