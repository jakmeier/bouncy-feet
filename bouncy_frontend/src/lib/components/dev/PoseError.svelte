<script>
  
  /**
   * @typedef {Object} Props
   * @property {import("bouncy_instructor").PoseApproximation} data
   */

  /** @type {Props} */
  let { data } = $props();
  let limbs = $derived(data.limbErrors());
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
      <li>
        Z-Error: {zError.error}{zError.quadrant_error ? ' wrong quadrant' : ''}
      </li>
    {/each}
    {#each data.zOrderErrors() as orderError}
      <li>
        Z-Order-Error: {orderError.expected}
      </li>
    {/each}
  </ol>
</div>
