<script>
  import { setContext, tick } from 'svelte';
  import { quadIn } from 'svelte/easing';
  import { tweened } from 'svelte/motion';
  import { derived, writable } from 'svelte/store';

  export let animationTime = 0;
  export let easing = quadIn;
  export let jumpHeight = 10;

  const animationTimeStore = writable(animationTime);
  $: animationTimeStore.set(animationTime);
  const easingStore = writable(easing);
  $: easingStore.set(easing);
  const animation = derived(
    [animationTimeStore, easingStore],
    ([$animationTime, $easing]) => {
      return {
        duration: $animationTime,
        easing: $easing,
      };
    }
  );
  // The timestamp when the current animation started, which we must know to
  // align the jump with the tweening.
  const animationTimeZero = writable(Date.now());

  // constructor for tweened jump stores
  const tweenedJump = (/** @type {number} */ start) => {
    const yStore = tweened(start, $animation);

    // This is the jump math.
    const createDerivedStore = () =>
      derived([yStore, animationTimeZero], ([$y, $timeZero], set) => {
        let t =
          ((Date.now() - $timeZero) % $animationTimeStore) /
          $animationTimeStore;
        // Parabolic jump with a peak at t = 0.5.
        // This might not be ideal for all moves but it's a good start.
        const jumpOffset = -4 * jumpHeight * t * (t - 1);
        set($y - jumpOffset);
      });

    // Return a custom store that can be user to set/get single numbers without
    // worrying about all the animation context.
    return {
      set: (/** @type {number} */ value) => yStore.set(value, $animation),
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

<slot />
