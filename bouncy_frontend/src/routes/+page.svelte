<script>
  import HomeFeedB from './HomeFeed_b.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

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
