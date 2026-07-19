<script>
  import ChallengeSetCard from '$lib/components/challenge/ChallengeSetCard.svelte';

  // @ts-nocheck
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  let { data } = $props();
</script>

<StandardPage mainColor title={'Pick your next **challenge**'}>
  {#if data.challengeSets.length === 0}
    <div class="empty">
      <span class="material-symbols-outlined">hourglass_top</span>
      <h2>New challenges are warming up</h2>
      <p>Check back soon for your next dance quest.</p>
    </div>
  {:else}
    <section class="sets" aria-label="Available challenge sets">
      {#each data.challengeSets as set, index}
        {#if set}
          <ChallengeSetCard {set} {index} />
        {/if}
      {/each}
    </section>
  {/if}
</StandardPage>

<style>
  .sets {
    display: grid;
    gap: 1rem;
  }
  .empty {
    margin-top: 1rem;
    padding: 3rem 1rem;
    border: 2px dashed var(--theme-neutral-gray);
    border-radius: 22px;
    text-align: center;
  }
  .empty > .material-symbols-outlined {
    font-size: var(--font-large);
  }
  .empty h2 {
    margin: 0.6rem 0;
    font-size: var(--font-normal);
  }
  .empty p {
    color: var(--theme-neutral-darker-gray);
    font-size: var(--font-small);
  }
</style>
