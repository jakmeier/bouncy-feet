import { readable } from 'svelte/store';

/**
 * Increase a counter at a fixed interval and provides it as a svelte store.
 *
 * TODO: To make this properly aligned with the music, setInterval is not
 * accurate. Consider using something else, or at least recompute the actual
 * delay to match the beat, rather than accumulating the error.
 *
 * @param {number} start
 * @param {number} increment
 * @param {number} ms
 */
export function counter(start, increment, ms) {
  let counterValue = start;
  const innerStore = readable(counterValue, (set) => {
    const handle = setInterval(() => {
      counterValue += increment;
      set(counterValue);
    }, ms);
    return () => clearInterval(handle);
  });

  return {
    subscribe: innerStore.subscribe,
  };
}

/**
 * Increase a counter at a dynamic interval.
 * 
 * The provided svelte store allows reading the value and also updating the update delay.
 *
 * @param {number} start
 * @param {number} increment
 * @param {number} ms
 */
export function dynamicCounter(start, increment, ms) {
  // A chain of `setTimeout` calls which uses live values of `delay` for the
  // delay. The `fuse` variable holds the handle of the currently outstanding
  // `setTimeout` call, which is canceled when the store is destroyed.
  /** @type { number } */
  let delay = ms;
  /** @type { number | NodeJS.Timeout | undefined} */
  let fuse;
  /** @type { number } */
  let counterValue = start;

  const innerStore = readable(counterValue, (set) => {
    const foo = () =>
      setTimeout(() => {
        counterValue += increment;
        set(counterValue);
        fuse = foo();
      }, delay);
    fuse = foo();
    return () => clearTimeout(fuse);
  });

  return {
    subscribe: innerStore.subscribe,
    setDelay: (/** @type {number} */ ms) => { delay = ms },
    set: (/** @type {number} */ value) => { counterValue = value },
    reset: () => { counterValue = -1 },
  };
}