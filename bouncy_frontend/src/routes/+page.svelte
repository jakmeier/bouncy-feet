<script>
  import { version } from '$lib/stores/FeatureSelection';
  import HomeFeed_001 from './HomeFeed_001.svelte';
  import HomeFeed_002 from './HomeFeed_002.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  let featuredDance =
    data.officialDances.length > 0 ? data.officialDances[0] : null;
</script>

{#if $version <= 0.001}
  <HomeFeed_001 {featuredDance} />
{:else}
  <HomeFeed_002 {featuredDance} />
{/if}
