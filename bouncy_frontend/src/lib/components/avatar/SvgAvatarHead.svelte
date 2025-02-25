<script>
  import SvgCircle from './SvgCircle.svelte';
  import SvgPolygon from './SvgPolygon.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} cx
   * @property {number} cy
   * @property {number} avatarSizePixels
   * @property {AvatarHeadStyle} headStyle
   * @property {String} color
   */

  /** @type {Props} */
  let { cx, cy, avatarSizePixels, headStyle, color } = $props();

  let r = 0.055 * avatarSizePixels * headStyle.size;
  let headFill = $derived(headStyle.shape === 'disk' ? color : 'transparent');
  let headStroke = $derived(headStyle.shape !== 'disk' ? color : undefined);
  let headStrokeWidth = $derived(
    headStyle.shape !== 'disk' ? (r / 2) * headStyle.strokeWidth : undefined
  );

  let corners = $derived([
    { x: cx - r, y: cy - r },
    { x: cx + r, y: cy - r },
    { x: cx + r, y: cy + r },
    { x: cx - r, y: cy + r },
  ]);
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
