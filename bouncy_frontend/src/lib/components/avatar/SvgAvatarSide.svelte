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
  /** @type{number} animationTime in ms */
  export let animationTime;

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

<SvgLine bind:animationTime start={hip} end={knee} z={side.thigh.z} />
<SvgLine bind:animationTime start={knee} end={ankle} z={side.shin.z} />
<SvgLine bind:animationTime start={shoulder} end={elbow} z={side.arm.z} />
<SvgLine bind:animationTime start={elbow} end={wrist} z={side.forearm.z} />
<SvgLine bind:animationTime start={heel} end={toe} z={side.foot.z} />
