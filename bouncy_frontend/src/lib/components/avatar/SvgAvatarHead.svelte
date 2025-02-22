<script>
  import SvgCircle from './SvgCircle.svelte';
  import SvgPolygon from './SvgPolygon.svelte';

  /** @type {number} */
  export let cx;
  /** @type {number} */
  export let cy;
  /** @type {number} */
  export let avatarSizePixels;
  /** @type {AvatarHeadStyle} */
  export let headStyle;
  /** @type {String} */
  export let color;
  export let dummyUpdate;

  let r = 0.055 * avatarSizePixels * headStyle.size;
  let headFill = headStyle.shape === 'disk' ? color : undefined;
  let headStroke = headStyle.shape !== 'disk' ? color : undefined;
  let headStrokeWidth =
    headStyle.shape !== 'disk' ? (r / 2) * headStyle.strokeWidth : undefined;

  $: corners = [
    { x: cx - r, y: cy - r },
    { x: cx + r, y: cy - r },
    { x: cx + r, y: cy + r },
    { x: cx - r, y: cy + r },
  ];
</script>

{#if headStyle.shape === 'disk' || headStyle.shape === 'circle'}
  <SvgCircle
    id="head"
    {cx}
    {cy}
    {r}
    fill={headFill}
    stroke={headStroke}
    strokeWidth={headStrokeWidth}
    {dummyUpdate}
  />
{:else if headStyle.shape === 'square'}
  <SvgPolygon
    id="poly-head"
    points={corners}
    style={{
      lineWidth: headStrokeWidth,
      color: headStroke,
      fill: headFill,
      linecap: 'round',
    }}
  />
{/if}
