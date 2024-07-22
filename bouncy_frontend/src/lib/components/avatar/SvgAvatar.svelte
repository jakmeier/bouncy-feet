<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import SvgPolygon from './SvgPolygon.svelte';
  import SvgStyle from './SvgStyle.svelte';
  import SvgCircle from './SvgCircle.svelte';
  import { Cartesian2d } from '$lib/instructor/bouncy_instructor';
  import { MAIN_THEME_COLORING } from '$lib/constants';

  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let bodyShift = { x: 0, y: 0 };
  export let avatarSize = 1.0;

  /** @type {AvatarColoring} */
  export let style = MAIN_THEME_COLORING;

  /**
   * @param {number} s
   * @param {number} min
   * @param {number} max
   */
  function wrap(s, min, max) {
    let wrapped = (s - min) % (max - min);
    if (wrapped < 0) {
      wrapped += max - min;
    }
    return wrapped + min;
  }

  $: avatarSizePixels = Math.min(height, width) * avatarSize;
  $: hip = {
    x: (0.5 + wrap(bodyShift.x, -0.75, 0.75)) * width,
    y: (0.5 + bodyShift.y) * height,
  };

  /** @type {Cartesian2d} */
  let leftHip;
  /** @type {Cartesian2d} */
  let rightHip;
  /** @type {Cartesian2d} */
  let rightShoulder;
  /** @type {Cartesian2d} */
  let leftShoulder;
  /** @type {Cartesian2d} */
  let headPosition;
  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonV2} */
  let renderedSkeleton;

  $: if (skeleton) {
    renderedSkeleton = skeleton.render(
      new Cartesian2d(hip.x, hip.y),
      avatarSizePixels
    );
    leftHip = renderedSkeleton.hip.start;
    leftShoulder = renderedSkeleton.shoulder.start;
    rightHip = renderedSkeleton.hip.end;
    rightShoulder = renderedSkeleton.shoulder.end;
    headPosition = new Cartesian2d(
      (leftShoulder.x + rightShoulder.x) / 2,
      leftShoulder.y - avatarSizePixels * 0.1
    );
  }

  $: headRadius = 0.075 * avatarSizePixels;
</script>

{#if skeleton}
  <SvgPolygon
    id="torso"
    points={[leftHip, rightHip, rightShoulder, leftShoulder]}
    style={{
      color: style.headColor,
      fill: style.bodyColor,
      linecap: 'round',
      lineWidth: lineWidth * 0.9,
    }}
  />
  <SvgCircle
    id="head"
    cx={headPosition.x}
    cy={headPosition.y}
    r={headRadius}
    fill={style.headColor}
  />
  <SvgStyle color={style.leftColor} linecap="round" {lineWidth}>
    <SvgAvatarSide side={renderedSkeleton.left} sideId={'left'}></SvgAvatarSide>
  </SvgStyle>
  <SvgStyle color={style.rightColor} linecap="round" {lineWidth}>
    <SvgAvatarSide side={renderedSkeleton.right} sideId={'right'}
    ></SvgAvatarSide>
  </SvgStyle>
{/if}
