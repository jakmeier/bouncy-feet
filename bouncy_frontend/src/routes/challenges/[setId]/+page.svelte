<script>
  import { getContext, onMount } from 'svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import ChallengeCard from '$lib/components/challenge/ChallengeCard.svelte';
  import {
    deriveChallengeProgress,
    readChallengeProgress,
    challengeProgressKey,
  } from '$lib/challengeProgress.js';
  import { publishChallengeDest } from '$lib/stores/UiState.svelte';
  import { asset } from '$app/paths';

  let { data } = $props();
  const { getSet } = getContext('challengeSets');
  const set = getSet(data.params.setId);
  const user = getContext('user');

  let progress = $derived(
    set
      ? readChallengeProgress(
          user.apiUser?.meta[challengeProgressKey(set.id)],
          set.definitionVersion
        )
      : null
  );
  let stateSummary = $derived(
    deriveChallengeProgress(set?.challenges ?? [], progress?.completed ?? {})
  );
  let completedCount = $derived(
    set
      ? set.challenges.filter(
          (
            /** @type {import('$lib/instructor/bouncy_instructor').Challenge} */ challenge
          ) => stateSummary.states[challenge.id].completed
        ).length
      : 0
  );
  let progressPercent = $derived(
    set?.challenges.length
      ? Math.round((completedCount / set.challenges.length) * 100)
      : 0
  );

  /** @param {import('$lib/instructor/bouncy_instructor').Challenge[]} challenges */
  function buildLevels(challenges) {
    /** @type {Record<string, number>} */
    const depths = {};
    const unresolved = [...challenges];
    let guard = challenges.length + 1;
    while (unresolved.length && guard-- > 0) {
      for (let index = unresolved.length - 1; index >= 0; index -= 1) {
        const challenge = unresolved[index];
        if (challenge.prerequisites.every((id) => depths[id] !== undefined)) {
          depths[challenge.id] = challenge.prerequisites.length
            ? Math.max(...challenge.prerequisites.map((id) => depths[id])) + 1
            : 0;
          unresolved.splice(index, 1);
        }
      }
    }
    return Object.values(
      challenges.reduce((levels, challenge) => {
        const depth = depths[challenge.id] ?? 0;
        levels[depth] ||= [];
        levels[depth].push(challenge);
        return levels;
      }, /** @type {Record<number, import('$lib/instructor/bouncy_instructor').Challenge[]>} */ ({}))
    );
  }

  const levels = set ? buildLevels(set.challenges) : [];
  /** @type {Map<string, number>} */
  const challengeNumbers = new Map(
    set?.challenges.map(
      (
        /** @type {import('$lib/instructor/bouncy_instructor').Challenge} */ challenge,
        /** @type {number} */ index
      ) => [challenge.id, index + 1]
    )
  );

  // Transition animation. No-op when the page is reached directly.
  onMount(() => {
    if (!set) return;
    requestAnimationFrame(() => {
      const titleNode = document.querySelector('header h1.title');
      const progressNode = document.querySelector('[data-challenge-progress]');
      publishChallengeDest(set.id, titleNode, progressNode);
    });
  });
</script>

<StandardPage black title={set ? set.name : '???'} backButton>
  {#if !set}
    <div class="empty-state">
      <span class="material-symbols-outlined">explore_off</span>
      <h1>Challenge set not found</h1>
      <a class="text-link" href="/challenges">See all challenges</a>
    </div>
  {:else}
    <div class="challenge-page">
      <header class="hero">
        <div
          class="progress-orbit"
          style={`--progress: ${progressPercent * 3.6}deg`}
        >
          <div class="progress-center">
            {#if stateSummary.won}
              <span class="material-symbols-outlined trophy">trophy</span>
            {:else}
              <strong>{completedCount}</strong>
              <small>of {set.challenges.length}</small>
            {/if}
          </div>
        </div>
        <div class="rewards">
          <span data-challenge-progress>
            0 / 1000
            <img
              src={asset('/img/symbols/bf_eye_black.svg')}
              alt=""
              class="currency"
            />
          </span>
          <span class="small"> 0 / 600 from challenges </span>
          <span class="small"> 0 / 400 completion bonus </span>
        </div>
      </header>

      {#if stateSummary.won}
        <div class="victory-banner">
          <span class="material-symbols-outlined">celebration</span>
          <div>
            <strong>Journey complete!</strong><br />You conquered every level.
          </div>
        </div>
      {:else}
        <div class="progress-copy">
          <span>Your journey</span>
          <span>{progressPercent}% complete</span>
        </div>
      {/if}

      <section class="path" aria-label="Challenge path">
        {#each levels as level, levelIndex}
          <div class="level" class:branch={level.length > 1}>
            {#each level as challenge}
              <ChallengeCard
                {challenge}
                state={stateSummary.states[challenge.id]}
                number={challengeNumbers.get(challenge.id) ?? 0}
                setId={set.id}
              />
            {/each}
          </div>
          {#if levelIndex < levels.length - 1}
            <div
              class="connector"
              class:split={levels[levelIndex + 1].length > 1}
              class:merge={level.length > 1}
              aria-hidden="true"
            >
              <span></span>
            </div>
          {/if}
        {/each}
      </section>
    </div>
  {/if}
</StandardPage>

<style>
  .challenge-page {
    max-width: 680px;
    margin: 0 auto;
    padding-bottom: 2rem;
  }

  .hero {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 1rem;
    overflow: hidden;
    padding: 1.4rem;
    border-radius: 24px;
    color: var(--theme-neutral-white);
    background: var(--theme-neutral-almost-black);
    box-shadow: 0 14px 35px var(--theme-neutral-dark-transparent);
  }

  .hero::before,
  .hero::after {
    content: '';
    position: absolute;
    border-radius: 999px;
    background: var(--theme-main);
    opacity: 0.1;
  }

  .hero::before {
    width: 180px;
    height: 180px;
    right: -70px;
    top: -95px;
  }
  .hero::after {
    width: 90px;
    height: 90px;
    left: 42%;
    bottom: -70px;
  }

  .rewards {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .rewards > span:first-child {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    align-self: stretch;
    gap: 0.35rem;
    margin-bottom: 0.6rem;
    padding: 0.3rem 0.65rem;
    border-radius: 999px;
    background: var(--theme-neutral-light);
    color: var(--theme-neutral-almost-black);
    font-size: var(--font-normal);
    font-weight: 800;
  }

  .small {
    font-size: var(--font-small);
  }

  .currency {
    position: relative;
    z-index: 1;
    width: 1rem;
    height: 1rem;
  }

  .progress-orbit {
    position: relative;
    z-index: 1;
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    width: 88px;
    height: 88px;
    border-radius: 50%;
    background: conic-gradient(
      var(--theme-main) var(--progress),
      var(--theme-main-60) 0
    );
  }

  .progress-orbit::after {
    content: '';
    position: absolute;
    inset: 7px;
    border-radius: 50%;
    background: var(--theme-neutral-almost-black);
  }
  .progress-center {
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    line-height: 1;
  }
  .progress-center strong {
    font-size: var(--font-normal);
  }
  .progress-center small {
    margin-top: 0.3rem;
    color: var(--theme-neutral-dark-gray);
    font-size: var(--font-small);
    text-transform: uppercase;
  }
  .progress-center .trophy {
    color: var(--theme-main);
    font-size: var(--font-large);
  }

  .progress-copy {
    display: flex;
    justify-content: space-between;
    margin: 1.5rem 0 1rem;
    color: var(--theme-neutral-darker-gray);
    font-size: var(--font-small);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.09em;
  }

  .victory-banner {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    margin: 1.25rem 0;
    padding: 0.9rem 1rem;
    border-radius: 16px;
    background: var(--theme-main-medium);
    font-size: var(--font-small);
  }
  .victory-banner .material-symbols-outlined {
    font-size: var(--font-large);
  }

  .path {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    padding: 0 0.25rem;
  }
  .level {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 0.7rem;
  }
  .level.branch {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .connector {
    position: relative;
    width: 100%;
    height: 34px;
  }
  .connector span {
    position: absolute;
    left: 50%;
    top: 0;
    width: 2px;
    height: 100%;
    transform: translateX(-50%);
    background: var(--theme-neutral-dark-gray);
  }
  .connector.split::after,
  .connector.merge::before {
    content: '';
    position: absolute;
    left: 25%;
    width: 50%;
    height: 17px;
    border-color: var(--theme-neutral-dark-gray);
    border-style: solid;
  }
  .connector.split::after {
    bottom: 0;
    border-width: 2px 2px 0;
    border-radius: 10px 10px 0 0;
  }
  .connector.merge::before {
    top: 0;
    border-width: 0 2px 2px;
    border-radius: 0 0 10px 10px;
  }

  .empty-state {
    padding: 4rem 1rem;
    text-align: center;
  }
  .empty-state > .material-symbols-outlined {
    font-size: var(--font-large);
  }
  .empty-state h1 {
    font-size: var(--font-large);
  }
  .text-link {
    color: var(--theme-main-dark);
    text-decoration: underline;
  }

  @media (max-width: 600px) {
    .hero {
      align-items: flex-start;
      padding: 1.2rem;
    }
    .progress-orbit {
      width: 70px;
      height: 70px;
    }
    .progress-orbit::after {
      inset: 6px;
    }
    .level.branch {
      gap: 0.45rem;
    }
  }
</style>
