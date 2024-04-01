<script>
  import { getContext, setContext, tick } from 'svelte';
  import PhysicalSvgLine from './PhysicalSvgLine.svelte';
  import PhysicalSvgPolygon from './PhysicalSvgPolygon.svelte';
  import PhysicalSvgCircle from './PhysicalSvgCircle.svelte';
  import SvgStyle from './SvgStyle.svelte';

  /** @type {number} */
  export let width;
  /** @type {number} */
  export let height;
  /** @type {boolean} */
  export let orderByZ = false;

  let animationCtx = getContext('animation');
  let animationTime = null;
  if (animationCtx) {
    animationTime = animationCtx.animationTime;
  }

  /**  @type {Line[]}  */
  let lines = [];
  /**  @type {Line[]}  */
  let displayedLines = [];
  /**  @type {Polygon[]}  */
  let polygons = [];
  /**  @type {Circle[]}  */
  let circles = [];

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

  async function update() {
    if (!orderByZ) {
      displayedLines = lines;
      return;
    }
    // Step 1: Set the z value according to the matching ID, then sort by z
    displayedLines = displayedLines
      .map((line) => {
        const matchedLine = lines.find((l) => l.id === line.id);
        if (matchedLine) {
          return { ...line, z: matchedLine.z };
        }
        return line;
      })
      .sort((a, b) => a.z - b.z);

    if (animationTime !== null) {
      const swap = $animationTime;
      $animationTime = 0;
      await tick();
      if (swap !== 0) {
        $animationTime = swap;
      }
    }

    // Step 2: Update also the other values, keeping the z-sorted order
    lines.forEach((line) => {
      const index = displayedLines.findIndex((l) => l.id === line.id);
      if (index !== -1) {
        displayedLines[index] = line;
      } else {
        displayedLines.push(line);
      }
    });
    displayedLines = displayedLines.sort((a, b) => a.z - b.z);
  }

  setContext('svg', { setLine, setPolygon, setCircle, update });
</script>

<svg viewBox="0 0 {width} {height}">
  <SvgStyle>
    <slot />
  </SvgStyle>

  <!-- For now, polygons and circles are drawn below lines. Z ordering can be added if needed -->
  {#each polygons as polygon}
    <PhysicalSvgPolygon points={polygon.points} style={polygon.style} />
  {/each}

  {#each circles as circle}
    <PhysicalSvgCircle
      cx={circle.cx}
      cy={circle.cy}
      r={circle.r}
      fill={circle.fill}
    />
  {/each}

  {#each displayedLines as line}
    <PhysicalSvgLine start={line.start} end={line.end} style={line.style} />
  {/each}
</svg>
