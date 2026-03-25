<script>
  /**
   * Seed-based wrapper around DancerCircle.
   *
   * Each seed generates a fully deterministic, unique logo.
   */

  import DancerCircle from './DancerCircle.svelte';
  import { PALETTES } from '$lib/palette';

  /**
   * @typedef {Object} Props
   * @property {boolean} isClub
   * @property {number} seed
   * @property {number} [size]
   * @property {string} [paletteId]
   */

  /** @type {Props} */
  let { isClub, seed, size = 192, paletteId } = $props();

  const params = $derived(derive(seed, paletteId));

  /**
   * @param {number} s
   * @param {string|undefined} pid
   */
  function derive(s, pid) {
    const rng = createRng(s);

    const palette = pid
      ? (PALETTES.find((p) => p.id === pid) ?? PALETTES[0])
      : PALETTES[rng.int(0, PALETTES.length - 1)];

    const count = isClub ? rng.int(2, 6) : 2;
    const radius = rng.range(22, 34);

    // Auto-scale so dancers don't overlap: arc between neighbours > dancer width
    // Dancer body width in SVG units are around 8.5 at scale 1.
    const maxScale = (2 * Math.PI * radius) / (count * 9.2);
    const baseScale = Math.min(maxScale, rng.range(2.8, 3.8));

    const startAngleDeg = rng.range(0, 360);

    const facingOptions = ['center', 'outward', 'free'];
    const facing = rng.pick(facingOptions);

    // Per-dancer config
    const shuffledBodies = rng.shuffle(palette.bodies);

    const bodyColors = Array.from(
      { length: count },
      (_, i) => shuffledBodies[i % shuffledBodies.length]
    );

    const scaleMultipliers = Array.from({ length: count }, () =>
      rng.range(0.75, 1.25)
    );
    const rotationOffsets = Array.from({ length: count }, () =>
      rng.range(-25, 25)
    );

    const showRing = true;
    const ringOpacity = rng.range(0.4, 0.9);
    const ringDashed = rng.next() > 0.6;
    const ringDashArray = ringDashed
      ? `${rng.range(1, 3).toFixed(1)} ${rng.range(0.5, 2).toFixed(1)}`
      : '';

    return {
      count,
      radius,
      baseScale,
      startAngleDeg,
      facing,
      bodyColors,
      backgroundColor: palette.background,
      ringColor: palette.ring,
      showRing,
      ringOpacity,
      ringDashArray,
      scaleMultipliers,
      rotationOffsets,
    };
  }

  /** Mulberry32
   * @param {number} seed
   */
  function createRng(seed) {
    let s = seed >>> 0;

    function next() {
      s += 0x6d2b79f5;
      let t = s;
      t = Math.imul(t ^ (t >>> 15), t | 1);
      t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    }

    return {
      next,
      range: (min, max) => min + next() * (max - min),
      int: (min, max) => Math.floor(min + next() * (max - min + 1)),
      pick: (arr) => arr[Math.floor(next() * arr.length)],
      shuffle: (arr) => {
        const a = [...arr];
        for (let i = a.length - 1; i > 0; i--) {
          const j = Math.floor(next() * (i + 1));
          [a[i], a[j]] = [a[j], a[i]];
        }
        return a;
      },
    };
  }
</script>

<DancerCircle
  count={params.count}
  radius={params.radius}
  baseScale={params.baseScale}
  startAngleDeg={params.startAngleDeg}
  facing={params.facing}
  bodyColors={params.bodyColors}
  backgroundColor={params.backgroundColor}
  ringColor={params.ringColor}
  showRing={params.showRing}
  ringOpacity={params.ringOpacity}
  ringDashArray={params.ringDashArray}
  scaleMultipliers={params.scaleMultipliers}
  rotationOffsets={params.rotationOffsets}
  {size}
/>
