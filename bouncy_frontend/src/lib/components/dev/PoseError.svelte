<script>
  /** @type {import("$lib/instructor/bouncy_instructor").PoseApproximation} */
  export let data;
  $: limbs = data.limbErrors();
  const precision = 3;
</script>

<div>
  {data.name} ({data.error.toPrecision(precision)}):
  <ol>
    {#each limbs as limbError}
      {#if limbError.weight > 0}
        <li>
          {limbError.name} | {limbError.error.toPrecision(precision)} (x{limbError.weight.toPrecision(
            2
          )})
        </li>
      {/if}
    {/each}
    {#each data.zErrors() as zError}
      <li>Z-Error, expected: {zError.expected}</li>
    {/each}
  </ol>
</div>
