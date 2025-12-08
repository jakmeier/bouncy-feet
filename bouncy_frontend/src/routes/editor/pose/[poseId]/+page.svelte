<script>
  import { page } from '$app/state';
  import PoseWeightsForm from '$lib/components/editor/PoseWeightsForm.svelte';
  import PoseAnglesForm from '$lib/components/editor/PoseAnglesForm.svelte';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import { t } from '$lib/i18n';
  import { beforeNavigate } from '$app/navigation';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  const poseId = page.params.poseId;

  const poseCtx = getContext('pose');
  const localCollectionCtx = getContext('localCollection');

  /** @type {Readable<PoseWrapper[]>} */
  const poses = localCollectionCtx.poses;

  /** @type {PoseAnglesForm} */
  let anglesForm;
  /** @type {PoseWeightsForm} */
  let weightsForm;

  let isDirty = $state(false);

  function copyPose() {
    let pose = anglesForm.readPose();
    if (pose) {
      weightsForm.loadPose(pose);
      isDirty = true;
    }
  }

  function savePose() {
    let pose = weightsForm.getPose();
    localCollectionCtx.addPose(pose);
    isDirty = false;
  }

  beforeNavigate(({ cancel }) => {
    if (isDirty) {
      if (!confirm($t('editor.confirm-leave'))) {
        cancel();
      }
    }
  });

  onMount(() => {
    let pose = $poses.find((p) => p.id() === poseId);
    if (pose) {
      anglesForm.loadPose(pose);
      weightsForm.loadPose(pose);
    }
  });
</script>

<LogoHeader
  title={$t('editor.pose.edit')}
  button="save"
  onAction={savePose}
  backButton
/>

<h2 class="box">{$t('editor.pose.angles-subtitle')}</h2>

<PoseAnglesForm bind:this={anglesForm} onChange={copyPose} />

<h2 class="box">{$t('editor.pose.weights-subtitle')}</h2>

<PoseWeightsForm bind:this={weightsForm} onChange={() => (isDirty = true)} />
