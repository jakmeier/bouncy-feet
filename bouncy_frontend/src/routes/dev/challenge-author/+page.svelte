<script>
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import ChallengeVideoAuthor from '$lib/components/editor/ChallengeVideoAuthor.svelte';

  let { data } = $props();
</script>

<BackHeader title="Challenge author" />
<ChallengeVideoAuthor
  initialVideoUrl={data.videoUrl}
  initialBeats={data.beats}
  initialStartTimestamp={data.startTimestamp}
/>
