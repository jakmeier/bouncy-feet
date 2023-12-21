<script>
  import SvgLine from './SvgLine.svelte';
  import { add2dVector } from '$lib/math';

  /** @type {{ x: number; y: number; }} */
  export let hip;
  /** @type {{ x: number; y: number; }} */
  export let shoulder;
  /** @type {import("$lib/instructor/bouncy_instructor").SkeletonSide} */
  export let side;
  /** @type {number} */
  export let size;
  /**
   * @type {{ thigh: number; shin: number; foot: number; arm: number; forearm: number; }}
   */
  export let lengths;

  $: knee = add2dVector(
    hip,
    side.thigh.angle,
    side.thigh.r * lengths.thigh * size
  );
  $: ankle = add2dVector(
    knee,
    side.shin.angle,
    side.shin.r * lengths.shin * size
  );
  // The foot is actually measured between heel and toe, but the skelton
  // doesn't give information about wrist to heel. I could do some
  // approximation, or actually track this. But seems not necessary. This
  // looks good enough for the 2d render.
  $: heel = ankle;
  $: toe = add2dVector(
    heel,
    side.foot.angle,
    side.foot.r * lengths.foot * size
  );
  $: elbow = add2dVector(
    shoulder,
    side.arm.angle,
    side.arm.r * lengths.arm * size
  );
  $: wrist = add2dVector(
    elbow,
    side.forearm.angle,
    side.forearm.r * lengths.forearm * size
  );
</script>

<SvgLine start={hip} end={knee}></SvgLine>
<SvgLine start={knee} end={ankle}></SvgLine>
<SvgLine start={shoulder} end={hip}></SvgLine>
<SvgLine start={shoulder} end={elbow}></SvgLine>
<SvgLine start={elbow} end={wrist}></SvgLine>
<SvgLine start={heel} end={toe}></SvgLine>
