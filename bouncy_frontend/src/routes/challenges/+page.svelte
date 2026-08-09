<script>
  import ChallengeSetList from '$lib/components/challenge/ChallengeSetList.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  let { data } = $props();
</script>

<StandardPage mainColor title={'Pick your next **challenge**'}>
  <ChallengeSetList sets={data.challengeSets} />
</StandardPage>
