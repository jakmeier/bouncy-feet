<script>
  import { version } from '$lib/stores/FeatureSelection';
  import HomeFeedA from './HomeFeed_a.svelte';
  import HomeFeedB from './HomeFeed_b.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  let featuredDance =
    data.officialDances.length > 0 ? data.officialDances[0] : null;
</script>

<!-- Switch between stable and unstable HomeFeed -->
{#if $version > 0.005}
  <HomeFeedA {featuredDance} />
{:else}
  <HomeFeedB {featuredDance} />
{/if}
