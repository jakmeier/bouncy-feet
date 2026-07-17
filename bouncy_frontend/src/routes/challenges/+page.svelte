<script>
  // @ts-nocheck
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  let { data } = $props();
</script>

<StandardPage white>
  <h1>Challenges</h1>
  {#if data.challengeSets.length === 0}<p>
      No challenge sets are available yet.
    </p>{/if}
  {#each data.challengeSets as set}<p>
      <a href={`/challenges/${set.id}`}>{set.name}</a>
    </p>{/each}
</StandardPage>
