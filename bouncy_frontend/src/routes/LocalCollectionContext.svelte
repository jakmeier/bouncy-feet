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
  import { derived, readable, writable } from 'svelte/store';

  const fileBuilder = browser
    ? DanceFileBuilder.fromRon(localStorage.dances)
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
    $builderStore = $builderStore.withDance(danceBuilder);
  }
</script>

<slot />
