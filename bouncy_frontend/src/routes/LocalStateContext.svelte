<script>
  /**
   * Local state, unlike user state, is not synchronized anywhere outside the local client.
   * Specific values might be synchronized indirectly, of course, but generally speaking, values here stay local.
   */

  import { browser } from '$app/environment';
  import { setContext } from 'svelte';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */
  /** @type {Props} */
  let { children } = $props();

  /** @type {AvatarStyleContext} */
  const defaultAvatarStyle = {
    coloring: {
      leftColor: 'var(--avatar-left)',
      rightColor: 'var(--avatar-right)',
      headColor: 'var(--avatar-head)',
    },
    headStyle: {
      shape: 'disk',
      size: 1,
      strokeWidth: 1,
    },
    bodyShape: {
      strokeWidth: 1,
    },
  };

  /** @type {LocalState} */
  const defaultState = {
    avatarStyle: defaultAvatarStyle,
  };

  /**
   * Load from localStorage or use defaults
   *
   * We do a deep merge. Every new object field must be added here.
   *
   * 1. Take default
   * 2. Overwrite all locally stored fields.
   * 3. Return the result.
   *
   * @return {LocalState}
   */
  function loadState() {
    const storedRaw = browser ? localStorage.getItem('localState') : undefined;
    const stored = storedRaw ? JSON.parse(storedRaw) : {};

    // deep copy of default
    const state = JSON.parse(JSON.stringify(defaultState));

    // assign is a shallow copy, every object must be called specifically.
    Object.assign(state.avatarStyle.coloring, stored?.avatarStyle?.coloring);
    Object.assign(state.avatarStyle.headStyle, stored?.avatarStyle?.headStyle);
    Object.assign(state.avatarStyle.bodyShape, stored?.avatarStyle?.bodyShape);
    return state;
  }

  // Export through bindable prop
  const localState = $state(loadState());

  // **Sync changes to localStorage**
  $effect(() =>
    // Note: stringify accesses all fields, hence $effect will track all changes
    localStorage.setItem('localState', JSON.stringify(localState))
  );

  setContext('localState', localState);
</script>

{@render children?.()}
