<script>
  /** @type{{ x: number; y: number; }[]} */
  export let points;
  /**
   * @type {{ lineWidth: any; color: any; fill: any; linecap: any; }}
   */
  export let style;

  $: coordinates = points.map((point) => `${point.x},${point.y}`).join(' ');
</script>

<polygon
  points={coordinates}
  stroke-width="{style.lineWidth}px"
  stroke={style.color}
  fill={style.fill}
  stroke-linecap={style.linecap}
/>
