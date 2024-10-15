<script>
  import Svg from '$lib/components/avatar/Svg.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import VideoToStep from '$lib/components/editor/VideoToStep.svelte';
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { getContext } from 'svelte';

  /** @type {import('./$types').PageData} */
  export let data;
  // const poses = data.lookupPoses();
  const localCollectionCtx = getContext('localCollection');
  const poses = localCollectionCtx.poses;
</script>

<div class="poses">
  {#each $poses as pose}
    <div class="pose">
      <p>{pose.name('en')}</p>
      <div class="avatar">
        <Svg width={200} height={200} orderByZ>
          <SvgAvatar
            skeleton={pose.skeleton()}
            width={200}
            height={200}
            style={LEFT_RIGHT_COLORING_LIGHT}
          ></SvgAvatar>
        </Svg>
      </div>
    </div>
  {/each}
</div>

<VideoToStep></VideoToStep>

<style>
  .poses {
    display: flex;
    flex-wrap: wrap;
  }
  .pose {
    max-width: 200px;
  }
</style>
