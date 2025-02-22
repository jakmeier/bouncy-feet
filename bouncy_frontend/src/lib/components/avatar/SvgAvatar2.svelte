<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import SvgPolygon from './SvgPolygon.svelte';
  import SvgStyle from './SvgStyle.svelte';
  import SvgCircle from './SvgCircle.svelte';
  import {
    Cartesian2d,
    RenderableSegment,
  } from '$lib/instructor/bouncy_instructor';
  import { MAIN_THEME_COLORING } from '$lib/constants';
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';

  /** @type import('$lib/instructor/bouncy_instructor').SkeletonV2 */
  export let skeleton;
  export let avatarSizePixels = 100;
  /** @type {RenderableSegment[]} */
  export let markedSegments = [];

  $: markedSegmentsLines = markedSegments.map((segment, i) => {
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
  });

  const markerColor = '#ff111166';
  const markerLineWidth = 16;

  /** @type {AvatarStyleContext} */
  const avatarStyleCtx = getContext('avatarStyle');

  /** @type {AvatarColoring} */
  let coloring = avatarStyleCtx.coloring;
  let lineWidth = 10 * avatarStyleCtx.bodyShape.strokeWidth;
  let headRadius = 0.055 * avatarSizePixels * avatarStyleCtx.headStyle.size;
  let headHeight = 1;
  let headFill =
    avatarStyleCtx.headStyle.shape === 'disk' ? coloring.headColor : undefined;
  let headStroke =
    avatarStyleCtx.headStyle.shape === 'circle'
      ? coloring.headColor
      : undefined;
  let headStrokeWidth =
    avatarStyleCtx.headStyle.shape === 'circle'
      ? (headRadius / 2) * avatarStyleCtx.headStyle.strokeWidth
      : undefined;

  /** @type {Cartesian2d} */
  $: leftHip = skeleton.hip.start;
  /** @type {Cartesian2d} */
  $: leftShoulder = skeleton.shoulder.start;
  /** @type {Cartesian2d} */
  $: rightHip = skeleton.hip.end;
  /** @type {Cartesian2d} */
  $: rightShoulder = skeleton.shoulder.end;
  /** @type {number} */
  $: cx = (leftShoulder.x + rightShoulder.x) / 2;
  /** @type {number} */
  $: cy = leftShoulder.y - (avatarSizePixels * headHeight) / 10;
  let dummyUpdate = 0;
  $: skeleton, (dummyUpdate += 1);

  $: console.log('headStrokeWidth', headStrokeWidth);
</script>

<SvgCircle
  id="head"
  {cx}
  {cy}
  r={headRadius}
  fill={headFill}
  stroke={headStroke}
  strokeWidth={headStrokeWidth}
  {dummyUpdate}
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
