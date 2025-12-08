<script>
  import VideoToPose from '$lib/components/editor/VideoToPose.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import { t } from '$lib/i18n';
</script>

<LogoHeader title={$t('editor.pose.new')} backButton />

<VideoToPose></VideoToPose>
