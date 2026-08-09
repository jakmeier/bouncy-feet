<script>
  import { t } from '$lib/i18n';
  import ChallengeSetCard from './ChallengeSetCard.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('$lib/instructor/bouncy_instructor').ChallengeSet[]} sets
   */

  /** @type {Props} */
  let { sets } = $props();
</script>

{#if sets.length === 0}
  <div class="empty">
    <span class="material-symbols-outlined">hourglass_top</span>
    <h2>{$t('challenges.empty-title')}</h2>
    <p>{$t('challenges.empty-description')}</p>
  </div>
{:else}
  <section class="sets" aria-label={$t('challenges.sets-label')}>
    {#each sets as set, index}
      {#if set}
        <ChallengeSetCard {set} {index} />
      {/if}
    {/each}
  </section>
{/if}

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
