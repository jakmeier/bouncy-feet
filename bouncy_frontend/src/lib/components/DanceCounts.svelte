<script>
  import { LEFT_RIGHT_COLORING_LIGHT } from '$lib/constants';
  import { DanceWrapper } from '$lib/instructor/bouncy_instructor';
  import Svg from './avatar/Svg.svelte';
  import SvgAvatar from './avatar/SvgAvatar.svelte';

  /** @type {DanceWrapper} */
  export let dance;

  let innerWidth = 300;

  $: beats = dance.beats;
  $: poseWidth = innerWidth / 8;

  function count(beat) {
    if (beat % 2 === 1) {
      return '+';
    } else {
      return (beat % 8) / 2 + 1;
    }
  }
</script>

<div class="poses" bind:clientWidth={innerWidth}>
  {#each { length: beats } as _, beat}
    <!-- <Pose /> -->
    <div class="avatar" style="width: {poseWidth}px">
      <div class="count">
        {count(beat)}
      </div>
      <Svg width={200} height={200} orderByZ>
        <SvgAvatar
          skeleton={dance.skeleton(beat)}
          width={200}
          height={200}
          style={LEFT_RIGHT_COLORING_LIGHT}
        ></SvgAvatar>
      </Svg>
    </div>
  {/each}
</div>

<style>
  .poses {
    display: grid;
    grid-template-columns: repeat(8, 1fr);
  }
  .avatar {
    position: relative;
    width: 100px;
    margin-bottom: 30px;
  }
  .count {
    text-align: center;
    margin: 0 0 -10px;
  }
</style>
