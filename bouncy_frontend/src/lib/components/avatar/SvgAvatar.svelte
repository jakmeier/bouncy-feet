<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import SvgPolygon from './SvgPolygon.svelte';
  import SvgStyle from './SvgStyle.svelte';
  import SvgCircle from './SvgCircle.svelte';
  import { add2dVector } from '$lib/math';

  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let bodyShift = { x: 0, y: 0 };
  export let lengths = {
    thigh: 0.2,
    shin: 0.2,
    torso: 0.25,
    arm: 0.1,
    forearm: 0.15,
    foot: 0.05,
    shoulder: 0.1,
    hip: 0.06,
  };

  export let leftColor = 'var(--theme-main)';
  export let rightColor = 'var(--theme-main)';
  export let headColor = 'var(--theme-main)';
  export let bodyColor = 'var(--theme-neutral-light)';

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

  $: sideway = skeleton ? skeleton.sideway : false;
  $: size = Math.min(height, width);
  $: hip = {
    x: (0.5 + wrap(bodyShift.x, -0.75, 0.75)) * width,
    y: (0.5 + bodyShift.y) * height,
  };
  $: shoulder = { x: hip.x, y: hip.y - lengths.torso * size };
  // $: shoulderLen = sideway ? 0.0 : 0.05 * size;
  // $: hipLen = sideway ? 0.0 : 0.03 * size;

  /** @type {{ x: number; y: number; }} */
  let leftHip;
  /** @type {{ x: number; y: number; }} */
  let rightHip;
  /** @type {{ x: number; y: number; }} */
  let rightShoulder;
  /** @type {{ x: number; y: number; }} */
  let leftShoulder;

  // right body part is left on screen
  $: {
    leftHip = add2dVector(
      hip,
      skeleton.hip.angle,
      -size * skeleton.hip.r * (lengths.hip / 2)
    );
    leftShoulder = add2dVector(
      shoulder,
      skeleton.shoulder.angle,
      -size * skeleton.shoulder.r * (lengths.shoulder / 2)
    );
    rightHip = add2dVector(
      hip,
      skeleton.hip.angle,
      size * skeleton.hip.r * (lengths.hip / 2)
    );
    rightShoulder = add2dVector(
      shoulder,
      skeleton.shoulder.angle,
      size * skeleton.shoulder.r * (lengths.shoulder / 2)
    );
    // when the dance looks away from the camera, we need to switch sides
    if (skeleton && skeleton.backwards) {
      [leftHip, rightHip] = [rightHip, leftHip];
      [leftShoulder, rightShoulder] = [rightShoulder, leftShoulder];
    }
  }

  $: headRadius = 0.075 * size;
</script>

<SvgPolygon
  id="torso"
  points={[leftHip, rightHip, rightShoulder, leftShoulder]}
  style={{
    color: headColor,
    fill: bodyColor,
    linecap: 'round',
    lineWidth: lineWidth * 0.9,
  }}
/>

{#if skeleton}
  <SvgCircle
    id="head"
    cx={shoulder.x}
    cy={shoulder.y - 0.1 * size}
    r={headRadius}
    fill={headColor}
  />
  <SvgStyle color={leftColor} linecap="round" {lineWidth}>
    <SvgAvatarSide
      {lengths}
      {size}
      side={skeleton.left}
      shoulder={leftShoulder}
      hip={leftHip}
      sideId={'left'}
    ></SvgAvatarSide>
  </SvgStyle>
  <SvgStyle color={rightColor} linecap="round" {lineWidth}>
    <SvgAvatarSide
      {lengths}
      {size}
      side={skeleton.right}
      shoulder={rightShoulder}
      hip={rightHip}
      sideId={'right'}
    ></SvgAvatarSide>
  </SvgStyle>
{/if}
