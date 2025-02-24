<script>
  import { getContext, setContext, tick } from 'svelte';
  import PhysicalSvgLine from './PhysicalSvgLine.svelte';
  import PhysicalSvgPolygon from './PhysicalSvgPolygon.svelte';
  import PhysicalSvgCircle from './PhysicalSvgCircle.svelte';
  import SvgStyle from './SvgStyle.svelte';
  import PhysicalSvgPath from './PhysicalSvgPath.svelte';

  
  
  
  
  /**
   * @typedef {Object} Props
   * @property {number} width
   * @property {number} height
   * @property {boolean} [orderByZ]
   * @property {boolean} [showOverflow]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    width,
    height,
    orderByZ = false,
    showOverflow = false,
    children
  } = $props();

  let animationCtx = getContext('animation');
  let animationTime = null;
  if (animationCtx) {
    animationTime = animationCtx.animationTime;
  }

  /**  @type {Line[]}  */
  let lines = [];
  /**  @type {Line[]}  */
  let displayedLines = $state([]);
  /**  @type {Path[]}  */
  let paths = [];
  /**  @type {Path[]}  */
  let displayedPaths = $state([]);
  /**  @type {Polygon[]}  */
  let polygons = $state([]);
  /**  @type {Circle[]}  */
  let circles = $state([]);

  /**
   * @param {string} id
   * @param {Line} line
   */
  function setLine(id, line) {
    const namedLine = { ...line, id };
    let index = lines.findIndex((x) => x.id === id);
    if (index !== -1) {
      lines[index] = namedLine;
    } else {
      lines.push(namedLine);
    }
  }

  /**
   * @param {string} id
   */
  function removeLine(id) {
    let index = lines.findIndex((x) => x.id === id);
    lines.splice(index, 1);
    lines = lines;
  }

  /**
   * @param {string} id
   * @param {Path} path
   */
  function setPath(id, path) {
    const namedPath = { ...path, id };
    let index = paths.findIndex((x) => x.id === id);
    if (index !== -1) {
      paths[index] = namedPath;
    } else {
      paths.push(namedPath);
    }
  }

  /**
   * @param {string} id
   */
  function removePath(id) {
    let index = paths.findIndex((x) => x.id === id);
    paths.splice(index, 1);
    paths = paths;
  }

  /**
   * @param {string} id
   * @param {Polygon} polygon
   */
  function setPolygon(id, polygon) {
    const namedPolygon = { ...polygon, id };
    let index = polygons.findIndex((x) => x.id === id);
    if (index !== -1) {
      polygons[index] = namedPolygon;
    } else {
      polygons.push(namedPolygon);
    }
  }

  /**
   * @param {string} id
   */
  function removePolygon(id) {
    let index = polygons.findIndex((x) => x.id === id);
    polygons.splice(index, 1);
    polygons = polygons;
  }

  /**
   * @param {string} id
   * @param {Circle} circle
   */
  function setCircle(id, circle) {
    const namedCircle = { ...circle, id };
    let index = circles.findIndex((x) => x.id === id);
    if (index !== -1) {
      circles[index] = namedCircle;
    } else {
      circles.push(namedCircle);
    }
  }

  /**
   * @param {string} id
   */
  function removeCircle(id) {
    let index = circles.findIndex((x) => x.id === id);
    circles.splice(index, 1);
    circles = circles;
  }

  async function update() {
    displayedLines = lines;
    displayedPaths = paths;
    if (orderByZ) {
      displayedLines = displayedLines.sort((a, b) => a.z - b.z);
      displayedPaths = displayedPaths.sort((a, b) => a.z - b.z);
    }
  }

  setContext('svg', {
    setLine,
    removeLine,
    setPath,
    removePath,
    setPolygon,
    removePolygon,
    setCircle,
    removeCircle,
    update,
  });
</script>

<svg
  viewBox="0 0 {width} {height}"
  style="overflow: {showOverflow ? 'visible' : 'hidden'};"
>
  <SvgStyle>
    {@render children?.()}
  </SvgStyle>

  <!-- For now, polygons and circles are drawn below lines. Z ordering can be added if needed -->
  {#each polygons as polygon}
    <PhysicalSvgPolygon points={polygon.points} style={polygon.style} />
  {/each}

  {#each circles as circle}
    <PhysicalSvgCircle {circle} />
  {/each}

  {#each displayedLines as line (line.id)}
    <PhysicalSvgLine start={line.start} end={line.end} style={line.style} />
  {/each}

  {#each displayedPaths as path (path.id)}
    <PhysicalSvgPath
      points={path.points}
      style={path.style}
    />
  {/each}
</svg>

<style>
  svg {
    width: 100%;
    height: 100%;
  }
</style>
