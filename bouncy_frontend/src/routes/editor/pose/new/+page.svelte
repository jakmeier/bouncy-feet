<script>
  import VideoToPose from '$lib/components/editor/VideoToPose.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
</script>

<Header title={$t('editor.pose.new')}></Header>

<VideoToPose></VideoToPose>
