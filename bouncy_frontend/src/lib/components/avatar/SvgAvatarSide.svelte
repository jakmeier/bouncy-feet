<script>
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';
  import SvgPath from './SvgPath.svelte';

  /**
   * @typedef {Object} Props
   * @property {import("$lib/instructor/bouncy_instructor").SkeletonSideV2} side
   * @property {any} sideId
   */

  /** @type {Props} */
  let { side, sideId } = $props();

  const svgStyle = getContext('svg-style');

  /**
   * @param {string} id
   * @param {import("$lib/instructor/bouncy_instructor").RenderableSegment} segment
   * @param {any} style
   * @returns {Line}
   */
  function svgLine(id, segment, style) {
    return {
      id,
      start: segment.start,
      end: segment.end,
      z: segment.z,
      style,
    };
  }

  /**
   * @param {string} id
   * @param {Point[]} points
   * @param {number} z
   * @param {Style} style
   * @returns {Path}
   */
  function svgPath(id, points, z, style) {
    return {
      id,
      points,
      z,
      style,
    };
  }

  // straight lines
  let lines = $derived([
    // svgLine('thigh', side.thigh, svgStyle),
    // svgLine('shin', side.shin, svgStyle),
    // svgLine('arm', side.arm, svgStyle),
    // svgLine('forearm', side.forearm, svgStyle),
    svgLine('foot', side.foot, svgStyle),
  ]);

  // bezier curves
  let paths = $derived([
    svgPath(
      'leg',
      [
        side.forearm.end,
        side.arm.end,
        side.arm.start,
        side.thigh.start,
        side.thigh.end,
        side.shin.end,
      ],
      side.thigh.z,
      svgStyle
    ),
  ]);
</script>

{#each lines as line}
  <SvgLine {line} id={`${sideId}-${line.id}`} />
{/each}

{#each paths as path}
  <SvgPath {path} id={`${sideId}-${path.id}`} />
{/each}
