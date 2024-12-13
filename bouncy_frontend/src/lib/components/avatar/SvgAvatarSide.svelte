<script>
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';
  import SvgPath from './SvgPath.svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonSideV2} */
  export let side;
  /** @type{string} */
  export let sideId;

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
   * @param {any} style
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
  $: lines = [
    // svgLine('thigh', side.thigh, $svgStyle),
    // svgLine('shin', side.shin, $svgStyle),
    // svgLine('arm', side.arm, $svgStyle),
    // svgLine('forearm', side.forearm, $svgStyle),
    svgLine('foot', side.foot, $svgStyle),
  ];

  // bezier curves
  $: paths = [
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
      $svgStyle
    ),
  ];
</script>

{#each lines as line}
  <SvgLine {line} id={`${sideId}-${line.id}`} />
{/each}

{#each paths as path}
  <SvgPath {path} id={`${sideId}-${path.id}`} />
{/each}
