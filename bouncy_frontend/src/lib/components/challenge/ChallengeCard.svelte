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
    <span class="state-icon material-symbols-outlined">
      {state.completed ? 'check' : state.locked ? 'lock' : 'play_arrow'}
    </span>
  </div>
  <h2>{challenge.name}</h2>
  <p>{challenge.explanation}</p>
  <span class="reward">
    <img src={asset('/img/symbols/bf_eye_black.svg')} alt="" class="currency" />
    0 / 100
  </span>
  <div class="card-meta">
    <span>
      <span class="material-symbols-outlined">music_note</span>
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
    border: 1.5px solid var(--theme-neutral-gray);
    border-radius: 18px;
    color: var(--theme-neutral-almost-black);
    background: var(--theme-neutral-white);
    box-shadow: 0 5px 14px var(--theme-neutral-dark-transparent);
    transition:
      transform 160ms ease,
      box-shadow 160ms ease,
      border-color 160ms ease;
  }

  a.challenge-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 9px 22px var(--theme-neutral-dark-transparent);
  }
  .challenge-card.available {
    border-color: var(--theme-main-alt);
    box-shadow: 0 7px 20px var(--theme-main-60);
  }
  .challenge-card.completed {
    background: var(--theme-main-light);
    border-color: var(--theme-main-medium);
  }
  .challenge-card.locked {
    color: var(--theme-neutral-darker-gray);
    background: var(--theme-neutral-light);
    border-color: transparent;
    box-shadow: none;
  }

  .card-topline,
  .card-meta,
  .card-meta > span {
    display: flex;
    align-items: center;
  }
  .card-topline,
  .card-meta {
    justify-content: space-between;
  }
  .card-meta > span {
    gap: 0.2rem;
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
    font-size: var(--font-normal);
  }
  .available .state-icon {
    color: var(--theme-neutral-almost-black);
    background: var(--theme-main);
  }
  .completed .state-icon {
    color: var(--theme-neutral-white);
    background: var(--theme-main-dark);
  }
  h2 {
    margin: 0.45rem 0 0.3rem;
    font-size: var(--font-normal);
    font-weight: 750;
    line-height: 1.15;
  }
  p {
    flex-grow: 1;
    margin: 0;
    color: var(--theme-neutral-darker-gray);
    font-size: var(--font-small);
    line-height: 1.4;
  }
  .reward {
    margin-top: 0.8rem;
    font-size: var(--font-small);
  }
  .currency {
    width: 1rem;
    height: 1rem;
    vertical-align: middle;
  }
  .card-meta {
    gap: 0.4rem;
    margin-top: 0.8rem;
    font-size: var(--font-small);
  }
  .card-meta .material-symbols-outlined {
    font-size: var(--font-small);
    color: var(--theme-main);
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
</style>
