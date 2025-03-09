<script>
  import { page } from '$app/state';
  import PoseWeightsForm from '$lib/components/editor/PoseWeightsForm.svelte';
  import PoseAnglesForm from '$lib/components/editor/PoseAnglesForm.svelte';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import { beforeNavigate } from '$app/navigation';

  const poseId = page.params.poseId;

  const poseCtx = getContext('pose');
  const localCollectionCtx = getContext('localCollection');

  /** @type {Readable<PoseWrapper[]>} */
  const poses = localCollectionCtx.poses;

  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let poseFromForm = $state();
  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").PoseWrapper)=>void} */
  let loadPoseToAngles = $state();
  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").PoseWrapper)=>void} */
  let loadPoseToWeights = $state();
  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let getPose = $state();

  let isDirty = $state(false);

  function copyPose() {
    let pose = poseFromForm();
    if (pose) {
      loadPoseToWeights(pose);
      isDirty = true;
    }
  }

  function savePose() {
    let pose = getPose();
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
      loadPoseToAngles(pose);
      loadPoseToWeights(pose);
    }
  });
</script>

<Header title={$t('editor.pose.edit')} button="save" on:click={savePose}
></Header>

<h2 class="box">{$t('editor.pose.angles-subtitle')}</h2>

<PoseAnglesForm
  bind:loadPose={loadPoseToAngles}
  bind:readPose={poseFromForm}
  onChange={copyPose}
/>

<h2 class="box">{$t('editor.pose.weights-subtitle')}</h2>

<PoseWeightsForm
  bind:loadPose={loadPoseToWeights}
  bind:getPose
  onChange={() => (isDirty = true)}
/>
