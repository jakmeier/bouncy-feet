<script>
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonSideV2} */
  export let side;
  /** @type{string} */
  export let sideId;

  const svgStyle = getContext('svg-style');

  /**
     * @param {string} id
     * @param {import("$lib/instructor/bouncy_instructor").RenderableSegment} segment
     * @param {any} style
     */
  function svgLine(id, segment, style) {
    console.log("svgline with style", style);
    return {
      id,
      start: segment.start,
      end: segment.end,
      z: segment.z,
      style,
    };
  }

  $: lines = [
    svgLine('thigh', side.thigh, $svgStyle),
    svgLine('shin', side.shin, $svgStyle),
    svgLine('arm', side.arm, $svgStyle),
    svgLine('forearm', side.forearm, $svgStyle),
    svgLine('foot', side.foot, $svgStyle),
  ];
</script>

{#each lines as line}
  <SvgLine {line} id={`${sideId}-${line.id}`} />
{/each}
