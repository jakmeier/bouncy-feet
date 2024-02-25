<script>
  /**
   * Provides access to locally stored content.
   */
  import { browser } from '$app/environment';
  import {
    DanceFileBuilder,
    DanceBuilder,
  } from '$lib/instructor/bouncy_instructor';
  import { setContext } from 'svelte';
  import { derived, writable } from 'svelte/store';

  const ron = browser ? localStorage.dances : null;
  const fileBuilder = ron
    ? DanceFileBuilder.fromRon(ron)
    : new DanceFileBuilder();

  const builderStore = writable(fileBuilder);
  const ctx = {
    builder: builderStore,
    dances: derived(builderStore, ($b) => $b.dances()),
    addDanceBuilder,
  };

  if (browser) {
    ctx.builder.subscribe(
      (/** @type {DanceFileBuilder} */ builder) =>
        (localStorage.dances = builder.buildRon())
    );
  }

  setContext('localCollection', ctx);

  /**
   * @param {DanceBuilder} danceBuilder
   */
  function addDanceBuilder(danceBuilder) {
    $builderStore.addDance(danceBuilder);
    // trigger update (can I do better?)
    $builderStore = $builderStore;
  }
</script>

<slot />
