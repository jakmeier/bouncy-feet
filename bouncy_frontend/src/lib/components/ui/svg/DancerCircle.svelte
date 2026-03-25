<script>
  import Dancer from './Dancer.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} count
   * @property {number} radius
   * @property {number} outerPad
   * @property {boolean} showRing
   * @property {string} ringColor
   * @property {number} ringWidth
   * @property {number} ringOpacity
   * @property {string} ringDashArray
   * @property {string} backgroundColor
   * @property {string[]} bodyColors
   * @property {number} baseScale
   * @property {number[]} scaleMultipliers
   * @property {number[]} rotationOffsets
   * @property {number} startAngleDeg
   * @property {'center'|'outward'|'free'} facing
   * @property {number} size
   */

  /** @type {Props}*/
  let {
    count = 3,
    radius = 28,
    outerPad = 18,
    showRing = true,
    ringColor = '#cbf3a3',
    ringWidth = 5.0,
    ringOpacity = 0.75,
    ringDashArray = '',
    backgroundColor = 'transparent',
    bodyColors = ['#a9eb66', '#ff7301', '#1d1d1b'],
    baseScale = 3.2,
    scaleMultipliers = [1],
    rotationOffsets = [0],
    startAngleDeg = 0,
    facing = 'center',
    size = 200,
  } = $props();

  let viewHalf = $derived(radius + outerPad);

  let dancers = $derived(
    Array.from({ length: count }, (_, i) => {
      const angle =
        ((2 * Math.PI) / count) * i + (startAngleDeg * Math.PI) / 180;
      const x = radius * Math.cos(angle);
      const y = radius * Math.sin(angle);

      const theta = angle * (180 / Math.PI);
      let baseRotation = 0;
      if (facing === 'center') baseRotation = theta - 90;
      if (facing === 'outward') baseRotation = theta + 90;
      // 'free' → 0 (controlled entirely by rotationOffsets)

      const sm = scaleMultipliers[i % scaleMultipliers.length] ?? 1;
      const ro = rotationOffsets[i % rotationOffsets.length] ?? 0;

      return {
        x,
        y,
        bodyColor: bodyColors[i % bodyColors.length],
        scale: baseScale * sm,
        rotation: baseRotation + ro,
      };
    })
  );
</script>

<svg
  width={size}
  height={size}
  viewBox="{-viewHalf} {-viewHalf} {viewHalf * 2} {viewHalf * 2}"
  xmlns="http://www.w3.org/2000/svg"
  aria-label="Dancer avatar"
  role="img"
>
  {#if backgroundColor !== 'transparent'}
    <rect
      x={-viewHalf}
      y={-viewHalf}
      width={viewHalf * 2}
      height={viewHalf * 2}
      fill={backgroundColor}
    />
  {/if}

  {#if showRing}
    <circle
      cx="0"
      cy="0"
      r={radius}
      fill="none"
      stroke={ringColor}
      stroke-width={ringWidth}
      stroke-opacity={ringOpacity}
      stroke-dasharray={ringDashArray || undefined}
    />
  {/if}

  <!-- Dancers -->
  {#each dancers as d}
    <g transform="translate({d.x}, {d.y})">
      <Dancer bodyColor={d.bodyColor} scale={d.scale} rotation={d.rotation} />
    </g>
  {/each}
</svg>
