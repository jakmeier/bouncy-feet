<script>
  import { page } from '$app/stores';
  import PoseEditForm from '$lib/components/editor/PoseEditForm.svelte';
  import PoseInputForm from '$lib/components/editor/PoseInputForm.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';
  import { getContext, onMount } from 'svelte';
  import { Readable } from 'svelte/store';

  const poseId = $page.params.poseId;

  const poseCtx = getContext('pose');
  const localCollectionCtx = getContext('localCollection');

  /** @type {Readable<PoseWrapper[]>} */
  const poses = localCollectionCtx.poses;

  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").SkeletonWrapper)=>void} */
  let loadSkeleton;
  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let poseFromForm;
  /** @type {(skeleton: import("$lib/instructor/bouncy_instructor").PoseWrapper)=>void} */
  let loadPose;
  /** @type {()=>import("$lib/instructor/bouncy_instructor").PoseWrapper} */
  let getPose;

  function copyPose() {
    let pose = poseFromForm();
    if (pose) {
      loadPose(pose);
    }
  }

  function savePose() {
    let pose = getPose();
    localCollectionCtx.addPose(pose);
  }

  onMount(() => {
    let pose = $poses.find((p) => p.id() === poseId);
    if (pose) {
      // TODO: need a way to load a skeleton wrapper! (it currently requires
      // keypoints, which is a bit annoying to get for a pose)
      // loadSkeleton(pose.skeleton());
    }
    // loadSkeleton()
  });
</script>

<PoseInputForm bind:loadSkeleton bind:readPose={poseFromForm}></PoseInputForm>

<button class="light full-width short" on:click={copyPose}> ↓ </button>

<PoseEditForm bind:loadPose bind:getPose></PoseEditForm>

<Button
  symbol="save"
  symbolSize={29}
  class="light full-width short"
  on:click={savePose}
></Button>
