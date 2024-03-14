<script>
  import { getContext, setContext, tick } from 'svelte';
  import PhysicalSvgLine from './PhysicalSvgLine.svelte';
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

  setContext('svg', { setLine, update });
</script>

<svg viewBox="0 0 {width} {height}">
  <SvgStyle>
    <slot />
  </SvgStyle>

  {#each displayedLines as line}
    <PhysicalSvgLine start={line.start} end={line.end} style={line.style} />
  {/each}
</svg>
