<script>
  import ChallengeSetList from '$lib/components/challenge/ChallengeSetList.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { t } from '$lib/i18n';
  let { data } = $props();
</script>

<StandardPage mainColor title={$t('challenges.title')}>
  <ChallengeSetList sets={data.challengeSets} />
</StandardPage>
