<script>
  import { asset } from '$app/paths';

  /**
   * @typedef {Object} ChallengeState
   * @property {boolean} completed
   * @property {boolean} available
   * @property {boolean} locked
   * @property {string[]} missing
   *
   * @typedef {Object} Props
   * @property {import('$lib/instructor/bouncy_instructor').Challenge} challenge
   * @property {ChallengeState} state
   * @property {number} number
   * @property {string} setId
   */

  /** @type {Props} */
  let { challenge, state, number, setId } = $props();

  const href = $derived(`/challenges/${setId}/${challenge.id}`);
  const status = $derived(
    state.completed ? 'completed' : state.locked ? 'locked' : 'available'
  );
  const stateSymbol = $derived(
    state.completed ? 'bf_check' : state.locked ? 'bf_lock' : 'bf_play'
  );
</script>

<svelte:element
  this={state.locked ? 'div' : 'a'}
  class="challenge-card"
  class:completed={state.completed}
  class:available={state.available}
  class:locked={state.locked}
  href={state.locked ? undefined : href}
  aria-label={`${challenge.name}, ${status}`}
>
  <div class="card-topline">
    <span class="level-number">LEVEL {number}</span>
    <span class="state-icon">
      <img
        src={asset(`/img/symbols/${stateSymbol}.svg`)}
        alt=""
        class="state-img"
      />
    </span>
  </div>
  <h2>{challenge.name}</h2>
  {#if challenge.explanation && challenge.explanation.length > 0}
    <p>{challenge.explanation}</p>
  {/if}
  <div class="card-meta">
    <span class="bubble bpm">
      <img src={asset('/img/symbols/bf_note.svg')} alt="" class="meta-icon" />
      {challenge.bpm} BPM
    </span>
    {#if state.completed}
      <strong>Done</strong>
    {:else if state.locked}
      <strong>Locked</strong>
    {:else}
      <strong>Let's go <span aria-hidden="true">→</span></strong>
    {/if}
  </div>
  {#if state.locked && state.missing.length > 1}
    <div class="requirement">Complete both paths to unlock</div>
  {/if}
</svelte:element>

<style>
  .challenge-card {
    position: relative;
    display: flex;
    flex-direction: column;
    min-width: 0;
    padding: 1rem 1.05rem;
    border-radius: 1.5rem;
    color: var(--theme-neutral-almost-black);
    background: var(--theme-neutral-white);
    transition: transform 160ms ease;
    container-type: inline-size;
  }

  a.challenge-card:hover {
    transform: translateY(-2px);
  }
  .challenge-card.available {
    background: var(--theme-main);
  }
  .challenge-card.completed {
    background: var(--theme-main-light);
  }
  .challenge-card.locked {
    color: var(--theme-neutral-darker-gray);
    background: var(--theme-neutral-light);
  }

  .card-topline,
  .card-meta {
    display: flex;
    align-items: center;
  }
  .card-topline,
  .card-meta {
    justify-content: space-between;
  }
  .level-number {
    color: var(--theme-neutral-darker-gray);
    font-size: var(--font-small);
    font-weight: 800;
    letter-spacing: 0.11em;
  }
  .state-icon {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--theme-neutral-gray);
  }
  .available .state-icon {
    background: var(--theme-neutral-light);
  }
  .completed .state-icon {
    background: var(--theme-main-dark);
  }
  .state-img {
    height: 18px;
    width: auto;
  }

  h2 {
    margin: 0.45rem 0 0.3rem;
    font-weight: 700;
    line-height: 1.15;
    text-align: center;
  }
  p {
    flex-grow: 1;
    margin: 0;
    line-height: 1.4;
  }
  .bubble {
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.25rem 0.55rem;
    border-radius: 9999px;
    background: var(--theme-neutral-white);
  }
  .meta-icon {
    height: 1rem;
    width: auto;
  }
  .card-meta {
    gap: 0.4rem;
    margin-top: 0.8rem;
    font-size: var(--font-small);
  }
  .card-meta strong {
    color: var(--theme-main-dark);
    white-space: nowrap;
  }
  .locked .card-meta strong {
    color: var(--theme-neutral-darker-gray);
  }
  .requirement {
    margin-top: 0.65rem;
    padding-top: 0.55rem;
    border-top: 1px solid var(--theme-neutral-gray);
    font-size: var(--font-small);
    text-align: center;
  }

  @media (max-width: 600px) {
    .challenge-card {
      padding: 0.85rem;
    }
  }

  @container (max-width: 14rem) {
    .card-meta {
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
    }
  }
</style>
