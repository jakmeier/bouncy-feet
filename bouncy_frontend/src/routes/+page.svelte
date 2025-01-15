<script>
  import { version } from '$lib/stores/FeatureSelection';
  import HomeFeedA from './HomeFeed_a.svelte';
  import HomeFeedB from './HomeFeed_b.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  let featuredDance =
    data.officialDances.length > 0 ? data.officialDances[0] : null;

  // (mockup) learn today step
  const featuredStep = data.lookupSteps({
    uniqueNames: true,
    sources: ['basic'],
  })[0];
</script>

<!-- Switch between stable and unstable HomeFeed -->
{#if $version <= 0.006}
  <HomeFeedA featuredDances={data.officialDances} />
{:else}
  <HomeFeedB featuredDances={data.officialDances} {featuredStep} />
{/if}
