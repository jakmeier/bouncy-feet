<script>
  import Arrow from '../ui/svg/Arrow.svelte';
  import UnstyledButton from '../ui/UnstyledButton.svelte';

  /**
   * @typedef {Object} Props
   * @property {UserSearchConfig} searchConfig --bindable
   */

  /** @type {Props} */
  let { searchConfig = $bindable() } = $props();

  function decPage() {
    searchConfig.page = Math.max(0, searchConfig.page - 1);
  }

  function incPage() {
    searchConfig.page += 1;
  }
</script>

<div class="page-number">
  <UnstyledButton onClick={decPage}>
    <div class="arrow left" class:disabled={searchConfig.page === 0}>
      <Arrow />
    </div>
  </UnstyledButton>
  <div>{searchConfig.page + 1}</div>
  <UnstyledButton onClick={incPage}>
    <div class="arrow right">
      <Arrow />
    </div>
  </UnstyledButton>
</div>

<style>
  .page-number {
    position: relative;
    display: grid;
    width: 100%;
    grid-template-columns: 1.2rem auto 1.2rem;
    margin: 1rem 0;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .arrow {
    max-width: 1.2rem;
    max-height: 2rem;
  }

  .left {
    rotate: 90deg;
  }
  .right {
    rotate: -90deg;
  }
</style>
