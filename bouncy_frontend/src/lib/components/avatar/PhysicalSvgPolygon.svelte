<script>
  import { tweened } from 'svelte/motion';
  import { getContext } from 'svelte';
  import { derived, writable } from 'svelte/store';

  /**
   * @typedef {Object} Props
   * @property {any} points
   * @property {{ lineWidth: any; color: any; fill: any; linecap: any; }} style
   */

  /** @type {Props} */
  let { points, style } = $props();

  let displayedPoints = JSON.parse(JSON.stringify(points));

  const animationCtx = getContext('animation');
  const animation = animationCtx
    ? animationCtx.animation
    : writable({ duration: 0 });

  const coordinates = createTweens(points);

  /**
   * @param {Point[]} ps
   */
  function updateValues(ps) {
    for (let i = 0; i < coordinates.length; i++) {
      // set tweens and let svelte motion handle the animation
      // subscribers are updating the displayedPoints
      coordinates[i].x.set(ps[i].x, $animation);
      coordinates[i].y.set(ps[i].y, $animation);
    }
  }

  const allStores = coordinates.flatMap((p) => [p.x, p.y]);
  const pointsList = derived(allStores, (_) =>
    displayedPoints.map((p) => `${p.x},${p.y}`).join()
  );

  /** @param {Point[]} p */
  function createTweens(p) {
    if (!Array.isArray(p)) {
      console.error('Points should be an array.');
      return [];
    }
    return p.map((point, i) => {
      const x = tweened(point.x, $animation);
      x.subscribe((x) => setX(i, x));
      const y = animationCtx
        ? animationCtx.tweenedJump(point.y)
        : writable(point.y);
      y.subscribe((/** @type {number} */ y) => setY(i, y));
      return {
        x,
        y,
      };
    });
  }

  /**
   * @param {number} i
   * @param {number} x
   */
  function setX(i, x) {
    displayedPoints[i].x = x;
  }
  /**
   * @param {number} i
   * @param {number} y
   */
  function setY(i, y) {
    displayedPoints[i].y = y;
  }
  $effect(() => updateValues(points));
</script>

<polygon
  points={$pointsList}
  stroke-width="{style.lineWidth}px"
  stroke={style.color}
  fill={style.fill}
  stroke-linecap={style.linecap}
/>
