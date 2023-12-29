<script>
  import SvgAvatarSide from './SvgAvatarSide.svelte';
  import SvgLine from './SvgLine.svelte';
  import SvgPolygon from './SvgPolygon.svelte';

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

  // right body part is left on screen
  $: leftHip = { x: hip.x + hipLen, y: hip.y };
  $: leftShoulder = { x: shoulder.x + shoulderLen, y: shoulder.y };
  $: rightHip = { x: hip.x - hipLen, y: hip.y };
  $: rightShoulder = { x: shoulder.x - shoulderLen, y: shoulder.y };

  $: headRadius = 0.075 * size;
</script>

{#if skeleton && !sideway}
  <g
    stroke-width="{lineWidth * 0.8}px"
    stroke={headColor}
    fill={bodyColor}
    stroke-linecap="round"
  >
    <SvgPolygon points={[leftHip, rightHip, rightShoulder, leftShoulder]} />
  </g>
{/if}

{#if skeleton}
  <g stroke-width="{lineWidth}px" stroke={leftColor} stroke-linecap="round">
    <SvgAvatarSide
      {lengths}
      {size}
      side={skeleton.left}
      shoulder={leftShoulder}
      hip={leftHip}
    ></SvgAvatarSide>
  </g>
  <g stroke-width="{lineWidth}px" stroke={rightColor} stroke-linecap="round">
    <SvgAvatarSide
      {lengths}
      {size}
      side={skeleton.right}
      shoulder={rightShoulder}
      hip={rightHip}
    ></SvgAvatarSide>
  </g>
  <circle
    cx={shoulder.x}
    cy={shoulder.y - 0.1 * size}
    r={headRadius}
    fill={headColor}
  />
{/if}
