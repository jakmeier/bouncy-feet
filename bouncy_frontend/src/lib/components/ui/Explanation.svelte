<script>
  import SpeechBubble from '$lib/components/ui/SpeechBubble.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import { Skeleton } from '$lib/instructor/bouncy_instructor';

  export let text;
  export let width = 300;
  const skeleton = Skeleton.resting(false);
  const tailSize = 18;
  $: figureWidth = (width * 2) / 3;
  $: bubbleTailRightOffset = `${figureWidth / 2 - 2 * tailSize}px`;

  /** @param {number} width */
  function selectLineWidth(width) {
    if (width <= 150) {
      return 2;
    }
    if (width <= 250) {
      return 5;
    }
    if (width <= 350) {
      return 8;
    }
    return 10;
  }
  $: lineWidth = selectLineWidth(width);
</script>

<div class="explanation" style="width: {width}px">
  <SpeechBubble
    {text}
    position="bottom"
    right={bubbleTailRightOffset}
    width={'100%'}
    {tailSize}
  />

  <div class="figure-cell" style="height: {figureWidth + tailSize}px">
    <div class="figure" style="max-width: {figureWidth}px;">
      <Svg width={figureWidth} height={figureWidth}>
        <SvgAvatar
          {skeleton}
          width={figureWidth}
          height={figureWidth}
          {lineWidth}
        ></SvgAvatar>
      </Svg>
    </div>
  </div>
</div>

<style>
  .figure-cell {
    /* for positioning the .figure */
    position: relative;
  }
  .figure {
    position: absolute;
    right: 0;
    margin-top: 7px;
  }
  .explanation {
    margin: auto;
  }
</style>
