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

  /** @type import('$lib/instructor/bouncy_instructor').SkeletonV2 */
  export let skeleton;
  /** @type {AvatarColoring} */
  export let style = MAIN_THEME_COLORING;
  export let lineWidth = 10;
  export let avatarSizePixels = 100;
  export let headRadius = 0.055 * avatarSizePixels;
  export let headHeight = 1;
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
  $: cy = leftShoulder.y - avatarSizePixels * headHeight / 10;
  let dummyUpdate = 0;
  $: skeleton, (dummyUpdate += 1);
</script>
<!-- 
<SvgPolygon
  id="torso"
  points={[leftHip, rightHip, rightShoulder, leftShoulder]}
  style={{
    color: style.headColor,
    fill: style.bodyColor,
    linecap: 'round',
    lineWidth: lineWidth * 0.9,
  }}
/> -->
<SvgCircle
  id="head"
  {cx}
  {cy}
  r={headRadius}
  fill={style.headColor}
  {dummyUpdate}
/>
<SvgStyle color={style.leftColor} linecap="round" {lineWidth}>
  <SvgAvatarSide side={skeleton.left} sideId={'left'}></SvgAvatarSide>
</SvgStyle>
<SvgStyle color={style.rightColor} linecap="round" {lineWidth}>
  <SvgAvatarSide side={skeleton.right} sideId={'right'}></SvgAvatarSide>
</SvgStyle>
{#each markedSegmentsLines as line}
  <SvgLine id={line.id} {line}></SvgLine>
{/each}
