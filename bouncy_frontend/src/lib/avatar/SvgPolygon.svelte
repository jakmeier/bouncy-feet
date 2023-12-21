<script>
  /** @type{{ x: number; y: number; }[]} */
  export let points;

  $: coordinates = points.map((point) => `${point.x},${point.y}`).join(' ');
</script>

<polygon points={coordinates} />
