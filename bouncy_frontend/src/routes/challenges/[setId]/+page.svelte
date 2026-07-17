<script>
  // @ts-nocheck
  import { getContext } from 'svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import {
    deriveChallengeProgress,
    readChallengeProgress,
    challengeProgressKey,
  } from '$lib/challengeProgress.js';
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
    set && deriveChallengeProgress(set.challenges, progress.completed)
  );
</script>

<StandardPage white>
  {#if !set}<h1>Challenge set not found</h1>
  {:else}<h1>{set.name}</h1>
    <p>{set.explanation || ''}</p>
    {#if stateSummary.won}<h2>Challenge set completed!</h2>{/if}
    {#each set.challenges as challenge}<section>
        <h3>{challenge.name}</h3>
        {#if stateSummary.states[challenge.id].completed}<p>Completed</p>
        {:else if stateSummary.states[challenge.id].locked}<p>
            Locked — complete {stateSummary.states[challenge.id].missing.join(
              ', '
            )}
          </p>
        {:else}<a href={`/challenges/${set.id}/${challenge.id}`}
            >Start challenge</a
          >{/if}
      </section>{/each}
  {/if}
</StandardPage>
