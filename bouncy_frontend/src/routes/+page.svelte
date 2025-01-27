<script>
  import { version } from '$lib/stores/FeatureSelection';
  import HomeFeedA from './HomeFeed_a.svelte';
  import HomeFeedB from './HomeFeed_b.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  let featuredDance =
    data.officialDances.length > 0 ? data.officialDances[0] : null;

  // (mockup) learn today step
  const featuredSteps = data
    .lookupSteps({
      uniqueNames: true,
      sources: ['basic', 'footwork'],
    })
    .filter((_, i) => (i & 1) == 0)
    .slice(0, 3);
  const idleStep = data.lookupSteps({
    uniqueNames: true,
    sources: ['idle_steps'],
  })[0];
</script>

<!-- Switch between stable and unstable HomeFeed -->
{#if $version <= 0.005}
  <HomeFeedA featuredDances={data.officialDances} />
{:else}
  <HomeFeedB featuredDances={data.officialDances} {featuredSteps} {idleStep} />
{/if}
