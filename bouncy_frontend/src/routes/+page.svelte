<script>
  import HomeFeedB from './HomeFeed_b.svelte';

  
  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  // (mockup) learn today step
  const featuredSteps = data
    .lookupSteps({
      uniqueNames: true,
      sources: ['basic', 'footwork'],
    })
    .filter((_, i) => (i & 1) == 0)
    .slice(0, 3);
</script>

<HomeFeedB featuredDances={data.officialDances} {featuredSteps} />
