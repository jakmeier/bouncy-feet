<script>
  import VideoToPose from '$lib/components/editor/VideoToPose.svelte';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import { t } from '$lib/i18n';
</script>

<BackHeader title={$t('editor.pose.new')} />

<VideoToPose></VideoToPose>
