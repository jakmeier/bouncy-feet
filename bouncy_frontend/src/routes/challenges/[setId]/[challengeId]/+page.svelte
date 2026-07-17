<script>
  // @ts-nocheck
  import { getContext, onDestroy } from 'svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import LiveActivity from '$lib/components/activity/LiveActivity.svelte';
  import { registerTracker, setBpm } from '$lib/stores/Beat';
  import { completeChallenge } from '$lib/challengeProgress.js';

  let { data } = $props();
  const { getSet } = getContext('challengeSets');
  const user = getContext('user');
  const set = getSet(data.params.setId);
  const challengeIndex = set?.challenges.findIndex(
    (item) => item.id === data.params.challengeId
  );
  const challenge =
    challengeIndex === undefined || challengeIndex < 0
      ? undefined
      : set.challenges[challengeIndex];

  let started = $state(false);
  let finished = $state(false);
  let succeeded = $state(false);
  let tracker = challenge ? set.tracker(challengeIndex) : undefined;
  // registerTracker establishes Svelte context, so it must run while this
  // component is being initialised (not later from the click handler).
  const unregisterTracker = tracker ? registerTracker(tracker) : undefined;

  function start() {
    if (!tracker || !challenge) return;
    // Challenges own their tempo; don't inherit the last practice activity's BPM.
    setBpm(challenge.bpm);
    started = true;
  }

  async function onDone(detection) {
    finished = true;
    // `forceBeat` records every expected beat. A complete challenge therefore
    // needs one match for every configured subbeat and no forced misses.
    succeeded =
      Boolean(detection) &&
      detection.poseMisses === 0 &&
      detection.poseMatches >= tracker.trackedSubbeats;
    if (succeeded && user.apiUser) {
      await completeChallenge(user.apiUser, set, challenge.id);
      user.apiUser.submitActivity(
        `challenge/${set.id}/${challenge.id}`,
        detection
      );
    }
  }

  function retry() {
    tracker.clear();
    finished = false;
    succeeded = false;
    started = false;
  }

  onDestroy(() => unregisterTracker?.());
</script>

<StandardPage white>
  {#if !challenge}
    <h1>Challenge not found</h1>
  {:else if !started}
    <a href={`/challenges/${set.id}`}>← Back to challenges</a>
    <h1>{challenge.name}</h1>
    <p>{challenge.explanation || ''}</p>
    <p><strong>{challenge.bpm} BPM</strong></p>
    <video
      controls
      src={challenge.solutionVideo.path()}
      aria-label={`${challenge.name} solution video`}
    ></video>
    <p>Complete all three jumping jacks in one uninterrupted, on-beat run.</p>
    <button onclick={start}>Start challenge</button>
  {:else if !finished}
    <div data-testid="challenge-live">
      <LiveActivity {onDone} teacherVideo={challenge.solutionVideo.path()} />
    </div>
  {:else}
    <h1>{succeeded ? 'Challenge completed!' : 'Not completed yet'}</h1>
    <p>
      {succeeded
        ? 'Your completion has been saved.'
        : 'Complete every move on beat in one run, then try again.'}
    </p>
    <button onclick={retry}>Try again</button>
    <a href={`/challenges/${set.id}`}>Back to challenges</a>
  {/if}
</StandardPage>

<style>
  video {
    max-width: 100%;
  }
  button {
    font: inherit;
    padding: 0.6rem 1rem;
  }
</style>
