<script>
  import { run as run_1 } from 'svelte/legacy';

  import { setContext, tick } from 'svelte';
  import { quadIn } from 'svelte/easing';
  import { tweened } from 'svelte/motion';
  import { derived, writable } from 'svelte/store';

  /**
   * @typedef {Object} Props
   * @property {number} [animationTime]
   * @property {number} [delay]
   * @property {any} [easing]
   * @property {number} [jumpHeight]
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let {
    animationTime = 0,
    delay = 0,
    easing = quadIn,
    jumpHeight = 1,
    children
  } = $props();

  // this factor is chosen to have a jump height of 1.0 to be a reasonable default
  const jumpHeightFactor = 0.025;

  const animationTimeStore = writable(animationTime);
  run_1(() => {
    animationTimeStore.set(animationTime);
  });
  const easingStore = writable(easing);
  run_1(() => {
    easingStore.set(easing);
  });
  const delayStore = writable(delay);
  run_1(() => {
    delayStore.set(delay);
  });

  const animation = derived(
    [animationTimeStore, delayStore, easingStore],
    ([$animationTime, $delay, $easing]) => {
      return {
        duration: $animationTime,
        delay: $delay,
        easing: $easing,
      };
    }
  );
  // The timestamp when the current animation started, which we must know to
  // align the jump with the tweening.
  const animationTimeZero = writable(performance.now());

  // constructor for tweened jump stores
  const tweenedJump = (/** @type {number} */ start) => {
    // yStore stores the (usually Y coordinate) value itself and an extra ID just
    // to hack around svelte optimizations. Without the id, the jump is only
    // shown if the value changes. But I need a jump from y=0 to y=0 to still be
    // a jump. By providing a unique id for each update, the tweening and
    // derived jump is triggered as needed.
    let id = 0;
    const yStore = tweened({ value: start, id }, $animation);

    // This is the jump math.
    const createDerivedStore = () =>
      derived([yStore, animationTimeZero], ([$y, $timeZero], set) => {
        let t =
          ((performance.now() - $timeZero) % $animationTimeStore) /
          $animationTimeStore;
        // Parabolic jump with a peak at t = 0.5.
        // This might not be ideal for all moves but it's a good start.
        const jumpOffset = -4 * jumpHeight * jumpHeightFactor * t * (t - 1);
        set($y.value - jumpOffset);
      });

    // Return a custom store that can be user to set/get single numbers without
    // worrying about all the animation context.
    return {
      set: (/** @type {number} */ value) => {
        id += 1;
        yStore.set({ value, id }, $animation);
      },
      subscribe: (
        /** @type {import("svelte/store").Subscriber<any>} */ run,
        /** @type {import("svelte/store").Invalidator<any> | undefined} */ invalidate
      ) => {
        return createDerivedStore().subscribe(run, invalidate);
      },
    };
  };

  setContext('animation', {
    // Duration in ms for a full transition.
    animationTime: animationTimeStore,
    // Writable easing function to be updated when necessary.
    easing: easingStore,
    // Complete animation object to use as arg to new tweened stores.
    animation,
    // Create a tweened jump from the animation context
    tweenedJump,
    // Update on every position change (TODO: can I make this less error-prone?)
    animationTimeZero,
  });
</script>

{@render children?.()}
