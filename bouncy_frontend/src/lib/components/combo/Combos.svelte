<script>
  /**
   * @typedef {Object} Props
   * @prop {ComboInfo[]} combos
   */

  /** @type {Props}*/
  let { combos } = $props();
</script>

{#each combos as combo}
  {combo.id}
  {combo.title}
{/each}
