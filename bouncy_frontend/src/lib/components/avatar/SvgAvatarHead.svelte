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

  let headRadius = 0.055 * avatarSizePixels * headStyle.size;
  let headFill = headStyle.shape === 'disk' ? color : undefined;
  let headStroke = headStyle.shape === 'circle' ? color : undefined;
  let headStrokeWidth =
    headStyle.shape === 'circle'
      ? (headRadius / 2) * headStyle.strokeWidth
      : undefined;
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
