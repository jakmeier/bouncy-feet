<script>
  import { run } from 'svelte/legacy';

  import { LEFT_RIGHT_COLORING } from '$lib/constants';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import Arrow from '../ui/Arrow.svelte';
  import AvatarStyleContext from './AvatarStyleContext.svelte';
  import Svg from './Svg.svelte';
  import SvgAvatar from './SvgAvatar.svelte';

  const skeleton = Skeleton.resting(false);
  const width = 100;
  const height = 100;

  const shapes = ['disk', 'circle', 'square'];
  let shapeIndex = $state(0);

  /** @type {AvatarHeadStyle} */
  let headStyle = $state(updateHeadstyle());
  /** @type {AvatarColoring} */
  let coloring = LEFT_RIGHT_COLORING;
  /** @type {AvatarBodyShape} */
  let bodyShape = { strokeWidth: 1.0 };

  function updateHeadstyle() {
    return {
      shape: shapes[shapeIndex],
      size: 1.0,
      strokeWidth: 1.0,
    };
  }

  /** @param {number} delta */
  function changeShape(delta) {
    shapeIndex = (shapeIndex + delta) % shapes.length;
    headStyle = updateHeadstyle();
  }

  run(() => {
    console.log(shapeIndex);
  });
</script>

<div class="container">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="left arrow" onclick={() => changeShape(-1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>
  <AvatarStyleContext {headStyle} {coloring} {bodyShape}>
    <Svg {width} {height}>
      <SvgAvatar {skeleton}></SvgAvatar>
    </Svg>
  </AvatarStyleContext>

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="arrow" onclick={() => changeShape(1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 2rem auto 2rem;
    align-items: baseline;
  }
  .arrow {
    max-width: 1rem;
  }
  .left {
    rotate: 180deg;
  }
</style>
