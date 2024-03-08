<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import PhysicalSvgLine from './PhysicalSvgLine.svelte';
  import SvgPolygon from './SvgPolygon.svelte';
  import SvgStyle from './SvgStyle.svelte';

  /** @type import('$lib/instructor/bouncy_instructor').Skeleton */
  export let skeleton;
  export let width = 100;
  export let height = 100;
  export let lineWidth = 10;
  export let lengths = {
    thigh: 0.2,
    shin: 0.2,
    torso: 0.25,
    arm: 0.1,
    forearm: 0.15,
    foot: 0.05,
  };

  export let leftColor = 'var(--theme-main)';
  export let rightColor = 'var(--theme-main)';
  export let headColor = 'var(--theme-main)';
  export let bodyColor = 'var(--theme-neutral-light)';

  $: sideway = skeleton ? skeleton.sideway : false;
  $: size = Math.min(height, width);
  $: hip = { x: 0.5 * width, y: 0.5 * height };
  $: shoulder = { x: hip.x, y: hip.y - lengths.torso * size };
  $: shoulderLen = sideway ? 0.0 : 0.05 * size;
  $: hipLen = sideway ? 0.0 : 0.03 * size;

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
    leftHip = { x: hip.x + hipLen, y: hip.y };
    leftShoulder = { x: shoulder.x + shoulderLen, y: shoulder.y };
    rightHip = { x: hip.x - hipLen, y: hip.y };
    rightShoulder = { x: shoulder.x - shoulderLen, y: shoulder.y };
    // when the dance looks away from the camera, we need to switch sides
    if (skeleton && skeleton.backwards) {
      [leftHip, rightHip] = [rightHip, leftHip];
      [leftShoulder, rightShoulder] = [rightShoulder, leftShoulder];
    }
  }

  $: headRadius = 0.075 * size;
</script>

{#if skeleton && !sideway}
  <SvgPolygon
    points={[leftHip, rightHip, rightShoulder, leftShoulder]}
    style={{
      color: headColor,
      fill: bodyColor,
      linecap: 'round',
      lineWidth: lineWidth * 0.9,
    }}
  />
{:else}
  <PhysicalSvgLine
    start={leftShoulder}
    end={leftHip}
    style={{
      color: headColor,
      linecap: 'round',
      lineWidth: lineWidth * 0.9,
    }}
  />
{/if}

{#if skeleton}
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
  <circle
    cx={shoulder.x}
    cy={shoulder.y - 0.1 * size}
    r={headRadius}
    fill={headColor}
  />
{/if}
