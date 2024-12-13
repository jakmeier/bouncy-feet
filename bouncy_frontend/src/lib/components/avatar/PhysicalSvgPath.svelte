<script>
  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { derived, writable } from 'svelte/store';

  /** @type{Point[]} */
  export let points;
  /** @type{number} animationTime in ms */

  /** @type{Style} */
  export let style;

  const animationCtx = getContext('animation');
  const animation = animationCtx
    ? animationCtx.animation
    : writable({ duration: 0 });
  const tweenedJump = animationCtx ? animationCtx.tweenedJump : writable;

  const tweenedPoints = points.map((point) => ({
    x: tweened(point.x, $animation),
    y: tweenedJump(point.y),
  }));

  const animationTimeZero = animationCtx
    ? animationCtx.animationTimeZero
    : null;

  /**
   * Create a smooth svg path, using Catmull–Rom splines
   * @param {Point[]} points
   * @returns {string}
   */
  function bezierSplines(points) {
    if (points.length < 2) return '';

    // First point
    let path = `M ${points[0].x} ${points[0].y}`;

    // All other points
    for (let i = 0; i < points.length - 1; i++) {
      const p0 = points[i - 1] || points[i];
      const p1 = points[i];
      const p2 = points[i + 1] || points[i];
      const p3 = points[i + 2] || p2;

      const c1 = {
        x: p1.x + (p2.x - p0.x) / 6,
        y: p1.y + (p2.y - p0.y) / 6,
      };

      const c2 = {
        x: p2.x - (p3.x - p1.x) / 6,
        y: p2.y - (p3.y - p1.y) / 6,
      };

      path += ` C ${c1.x} ${c1.y}, ${c2.x} ${c2.y}, ${p2.x} ${p2.y}`;
    }

    return path;
  }

  const pathString = derived(
    tweenedPoints.map((point) => [point.x, point.y]).flat(),
    ($points) => {
      const formattedPoints = [];
      for (let i = 0; i < $points.length; i += 2) {
        formattedPoints.push({ x: $points[i], y: $points[i + 1] });
      }
      return bezierSplines(formattedPoints);
    }
  );

  // listen to prop changes and then update tweens
  function updatePosition() {
    tweenedPoints.forEach((tweenedPoint, i) => {
      tweenedPoint.x.set(points[i].x, $animation);
      tweenedPoint.y.set(points[i].y);
    });

    if (animationTimeZero) {
      animationTimeZero.set(performance.now());
    }
  }
  $: points, updatePosition();
</script>

<path
  d={$pathString}
  stroke-width="{style.lineWidth}px"
  stroke={style.color}
  stroke-linecap={style.linecap}
  fill="none"
>
</path>

<path stroke="blue" fill="none" stroke-width="4" />
