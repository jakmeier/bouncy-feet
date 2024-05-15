<script>
  import { setContext, tick } from 'svelte';
  import { quadIn } from 'svelte/easing';
  import { derived, writable } from 'svelte/store';

  export let animationTime = 0;
  export let easing = quadIn;

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

  setContext('animation', {
    animationTime: animationTimeStore,
    easing: easingStore,
    animation,
  });
</script>

<slot />
