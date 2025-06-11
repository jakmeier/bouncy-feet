<script>
  import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from '$lib/constants';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';
  import { getContext } from 'svelte';
  import Arrow from '../ui/Arrow.svelte';
  import AvatarStyleContext from './AvatarStyleContext.svelte';
  import Svg from './Svg.svelte';
  import SvgAvatar from './SvgAvatar.svelte';

  /** @type {LocalState}*/
  const localState = getContext('localState');

  const skeleton = Skeleton.resting(false);
  const width = 250;
  const height = 300;

  const shapes = ['disk', 'circle', 'square'];
  const strokeWidths = [1.0, 0.75, 1.25, 1.5];
  const colorings = [LEFT_RIGHT_COLORING, ORANGE_COLORING];
  let shapeIndex = $state(0);
  let strokeWidthIndex = $state(0);
  let coloringIndex = $state(0);

  /** @type {AvatarHeadStyle} */
  let headStyle = $derived({
    shape: shapes[shapeIndex],
    size: 1.0,
    strokeWidth: 1.0,
  });
  /** @type {AvatarColoring} */
  let coloring = $derived(colorings[coloringIndex]);
  /** @type {AvatarBodyShape} */
  let bodyShape = $derived({ strokeWidth: strokeWidths[strokeWidthIndex] });

  /** @param {number} delta */
  function changeShape(delta) {
    shapeIndex = (shapeIndex + delta) % shapes.length;
  }

  /** @param {number} delta */
  function changeStrokeWidth(delta) {
    strokeWidthIndex = (strokeWidthIndex + delta) % strokeWidths.length;
  }

  /** @param {number} delta */
  function changeColoring(delta) {
    coloringIndex = (coloringIndex + delta) % colorings.length;
  }

  function save() {
    Object.assign(localState.avatarStyle.headStyle, headStyle);
    Object.assign(localState.avatarStyle.coloring, coloring);
    Object.assign(localState.avatarStyle.bodyShape, bodyShape);
  }
</script>

<div class="container">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="left arrow" onclick={() => changeShape(-1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>
  <div class="avatar">
    <AvatarStyleContext {headStyle} {coloring} {bodyShape}>
      <Svg {width} {height}>
        <SvgAvatar {skeleton} {width} {height}></SvgAvatar>
      </Svg>
    </AvatarStyleContext>
  </div>

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="right arrow" onclick={() => changeShape(1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="left arrow" onclick={() => changeStrokeWidth(-1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="right arrow" onclick={() => changeStrokeWidth(1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="left arrow" onclick={() => changeColoring(-1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="right arrow" onclick={() => changeColoring(1)}>
    <Arrow color="var(--theme-neutral-white)" />
  </div>

  <div class="button">
    <!-- WIP -->
    <!-- TODO: translate -->
    <button onclick={save}>save</button>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 2rem auto 2rem;
    width: 20rem;
    height: min-content;
    margin: auto;
    overflow: auto;
  }
  .avatar {
    grid-column: 2;
    grid-row: 1 / span 3;
  }
  .arrow {
    max-width: 1.2rem;
    max-height: 2rem;
  }
  .left {
    rotate: 90deg;
  }
  .right {
    rotate: -90deg;
  }
  .button {
    grid-column: 1 / span 3;
    margin: auto;
  }
</style>
