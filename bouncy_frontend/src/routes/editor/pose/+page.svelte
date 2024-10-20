<script>
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { getContext } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { t } from '$lib/i18n';
  import Pose from '$lib/components/Pose.svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  // const poses = data.lookupPoses();
  const localCollectionCtx = getContext('localCollection');
  const poses = localCollectionCtx.poses;
</script>

<Header title={$t('editor.pose.title')}></Header>

<div class="centered">
  <a href="./new">
    <button class="light big wide"> {$t('editor.pose.new')} </button>
  </a>
</div>

<h2 class="box">{$t('editor.pose.list')}</h2>

<div class="poses">
  {#each $poses as pose}
    <div class="pose">
      <Pose {pose}></Pose>
    </div>
  {/each}
</div>

<style>
  .poses {
    display: flex;
    flex-wrap: wrap;
  }
  .pose {
    max-width: 200px;
  }
  .centered {
    margin: 30px auto;
    text-align: center;
  }
</style>
