<script>
  import SvgLine from './SvgLine.svelte';
  import { getContext } from 'svelte';

  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonSideV2} */
  export let side;
  /** @type{string} */
  export let sideId;

  const svg = getContext('svg');
  const svgStyle = getContext('svg-style');

  /**
   * @param {string} id
   * @param {import("$lib/instructor/bouncy_instructor").RenderableSegment} segment
   */
  function svgLine(id, segment) {
    return {
      id,
      start: segment.start,
      end: segment.end,
      z: segment.z,
      style: svgStyle,
    };
  }

  $: lines = [
    svgLine('thigh', side.thigh),
    svgLine('shin', side.shin),
    svgLine('arm', side.arm),
    svgLine('forearm', side.forearm),
    svgLine('foot', side.foot),
  ];
</script>

{#each lines as line}
  <SvgLine {line} id={`${sideId}-${line.id}`} />
{/each}
