<script>
  import { getContext } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import Pose from '$lib/components/Pose.svelte';
  import EditOrDeleteList from '$lib/components/ui/EditOrDeleteList.svelte';
  import Popup from '$lib/components/ui/Popup.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { writable } from 'svelte/store';
  import { PoseFileWrapper } from '$lib/instructor/bouncy_instructor';
  import { downloadTextFile } from '$lib/text_utils';
  import { goto } from '$app/navigation';

  /** @type {import('./$types').PageData} */
  export let data;
  // const poses = data.lookupPoses();
  const localCollectionCtx = getContext('localCollection');
  const poses = localCollectionCtx.poses;
  const builder = localCollectionCtx.poseBuilder;
  let showSettings = writable(false);

  /** @param {number} index */
  function editPose(index) {
    const poseId = $poses[index].id();
    goto(`./${poseId}`);
  }

  /** @param {number} index */
  function deleteConfirmed(index) {
    const poseId = $poses[index].id();
    try {
      localCollectionCtx.removePose(poseId);
    } catch (error) {
      console.warn('deleting pose failed', poseId, error);
      alert($t('editor.pose.delete-failed'));
    }
  }

  function openSettings() {
    $showSettings = true;
  }

  function handleFileUpload(event) {
    const file = event.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const ronString = e.target.result;
        importFile(ronString);
      };
      reader.readAsText(file);
    }
  }

  /** @param {string} ronString */
  function importFile(ronString) {
    try {
      const file = PoseFileWrapper.fromRon(ronString);
      for (let pose of file.poses()) {
        localCollectionCtx.addPose(pose);
      }
    } catch (e) {
      alert(e);
    }
  }

  function exportFile() {
    const string = $builder.buildPrettyRon();
    downloadTextFile('my_poses.poses.ron', string);
  }
</script>

<Header title={$t('editor.pose.title')} button="menu" on:click={openSettings} />

<div class="centered">
  <a href="./new">
    <button class="big wide"> {$t('editor.pose.new')} </button>
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

<Popup title={'editor.settings'} bind:isOpen={showSettings} showOkButton>
  <Button
    class="wide"
    symbol="upgrade"
    text={'editor.pose.export-all'}
    on:click={exportFile}
  />
  <Button
    class="wide"
    symbol="system_update_alt"
    text={'editor.pose.import'}
    on:click={() => document.querySelector('input#ron-upload')?.click()}
  ></Button>
</Popup>

<div class="hidden">
  <input
    id="ron-upload"
    type="file"
    accept=".ron"
    on:change={handleFileUpload}
  />
</div>

<style>
  .pose {
    max-width: 200px;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }
  .hidden {
    display: None;
  }
</style>
