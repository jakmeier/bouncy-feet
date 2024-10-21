<script>
  import { getContext } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import Pose from '$lib/components/Pose.svelte';
  import EditOrDeleteList from '$lib/components/ui/EditOrDeleteList.svelte';
  import { goto } from '$app/navigation';

  /** @type {import('./$types').PageData} */
  export let data;
  // const poses = data.lookupPoses();
  const localCollectionCtx = getContext('localCollection');
  const poses = localCollectionCtx.poses;

  /** @param {number} index */
  function editPose(index) {
    // const poseId = $poses[index].id();
    // goto(`./${poseId}`);

    // loading of the pose isn't properly implemented
    console.warn('editing poses not implemented');
    alert('editing poses not implemented :(');
  }

  /** @param {number} index */
  function deleteConfirmed(index) {
    // const poseId = $poses[index].id();
    // localCollectionCtx.removePose(poseId);

    // before deleting, it should be checked that no step uses the pose!
    console.warn('deleting not implemented');
    alert('deleting poses not implemented :(');
  }
</script>

<Header title={$t('editor.pose.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.pose.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.pose.list')}</h2>

<EditOrDeleteList items={$poses} onDelete={deleteConfirmed} onEdit={editPose}>
  <div slot="item" let:item={pose} let:selected>
    <div class="pose">
      <div>{pose.name('en')}</div>
      <Pose {pose}></Pose>
    </div>
  </div>
  <div slot="confirm-delete-text" let:item={pose}>
    <p>
      {$t('editor.pose.delete-confirmation0')}
      "{pose.name('en')}"
      {$t('editor.pose.delete-confirmation1')}
    </p>
  </div>
</EditOrDeleteList>

<style>
  .pose {
    max-width: 200px;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }
</style>
