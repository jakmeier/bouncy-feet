<script>
  import { getContext, onDestroy } from 'svelte';
  import { goto } from '$app/navigation';
  import { dev } from '$lib/stores/FeatureSelection';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import VideoWithMetaData from '$lib/components/ui/video/VideoWithMetaData.svelte';
  import LiveActivity from '$lib/components/activity/LiveActivity.svelte';
  import ActivityReview from '$lib/components/activity/ActivityReview.svelte';
  import { registerTracker, setBpm } from '$lib/stores/Beat';
  import { songs } from '$lib/stores/Songs';
  import { resolveSongTiming } from '$lib/songTiming.js';
  import { completeChallenge } from '$lib/challengeProgress.js';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import { t } from '$lib/i18n';

  let { data } = $props();
  const { getSet } = getContext('challengeSets');
  const set = getSet(data.params.setId);
  const challengeIndex = set?.challenges.findIndex(
    (
      /** @type {import('$lib/instructor/bouncy_instructor').Challenge} */ item
    ) => item.id === data.params.challengeId
  );
  const challenge =
    challengeIndex === undefined || challengeIndex < 0
      ? undefined
      : set.challenges[challengeIndex];

  let started = $state(false);
  /** @type {ApiUser | undefined} */
  let activityUser = $state();
  let finished = $state(false);
  let succeeded = $state(false);
  let savingCompletion = $state(false);
  let completionSaved = $state(false);
  let completionSyncPending = $state(false);
  let invalidChallengeSong = $state(false);
  /** @type {import('$lib/instructor/bouncy_instructor').DetectionResult | undefined} */
  let detection = $state();
  /** @type {number | undefined} */
  let recordingStart = $state();
  /** @type {number | undefined} */
  let recordingEnd = $state();
  /** @type {string | undefined} */
  let reviewVideoSrc = $state();
  /** @type {string[]} */
  const configuredSongIds = challenge?.allowedSongs?.length
    ? challenge.allowedSongs
    : challenge?.defaultSong
      ? [challenge.defaultSong]
      : [];
  /** @type {Song[]} */
  const availableSongs = [];
  for (const songId of configuredSongIds) {
    const song = songs.get(songId);
    if (!song || song.bpm !== challenge?.bpm) continue;
    availableSongs.push(song);
  }
  let selectedSongId = $state(
    availableSongs.some((song) => song.id === challenge?.defaultSong)
      ? challenge.defaultSong
      : availableSongs[0]?.id
  );
  let selectedSong = $derived(
    availableSongs.find((song) => song.id === selectedSongId)
  );
  let songTiming = $derived(
    selectedSong
      ? resolveSongTiming(
          selectedSong,
          challenge?.songTimingFor(selectedSong.id)
        )
      : undefined
  );
  let tracker = challenge ? set.tracker(challengeIndex) : undefined;
  // registerTracker establishes Svelte context, so it must run while this
  // component is being initialised (not later from the click handler).
  const unregisterTracker = tracker ? registerTracker(tracker) : undefined;

  /** @param {ApiUser} apiUser */
  async function start(apiUser) {
    if (!tracker || !challenge || !selectedSongId) return;
    activityUser = apiUser;
    // Challenges own their tempo; don't inherit the last practice activity's BPM.
    setBpm(challenge.bpm);
    started = true;
  }

  /**
   * @param {import('$lib/instructor/bouncy_instructor').DetectionResult} newDetection
   * @param {number} newRecordingStart
   * @param {number} newRecordingEnd
   * @param {string} newVideoUrl
   * @param {{ succeeded: boolean, invalidChallengeSong: boolean }} outcome
   */
  async function onDone(
    newDetection,
    newRecordingStart,
    newRecordingEnd,
    newVideoUrl,
    outcome
  ) {
    detection = newDetection;
    recordingStart = newRecordingStart;
    recordingEnd = newRecordingEnd;
    reviewVideoSrc = newVideoUrl;
    finished = true;

    // Challenges permit misses and can continue past their first pattern.
    // LiveActivity owns the duration-and-hit-threshold decision.
    succeeded = Boolean(detection) && outcome?.succeeded === true;
    invalidChallengeSong = outcome?.invalidChallengeSong === true;
    if (succeeded && activityUser) {
      savingCompletion = true;
      try {
        const completion = await completeChallenge(
          activityUser,
          set,
          challenge.id
        );
        completionSaved = true;
        completionSyncPending = completion.syncPending;
      } catch (error) {
        console.error('Could not save challenge completion', error);
      } finally {
        savingCompletion = false;
      }
    }
    // Retain the complete score for both successful and song-ended attempts.
    if (activityUser && detection) {
      activityUser.submitActivity(
        `challenge/${set.id}/${challenge.id}`,
        detection
      );
    }
  }

  function clearReview() {
    if (reviewVideoSrc) URL.revokeObjectURL(reviewVideoSrc);
    detection = undefined;
    recordingStart = undefined;
    recordingEnd = undefined;
    reviewVideoSrc = undefined;
  }

  function retry() {
    tracker.clear();
    clearReview();
    finished = false;
    succeeded = false;
    savingCompletion = false;
    completionSaved = false;
    completionSyncPending = false;
    invalidChallengeSong = false;
    started = false;
  }

  /** @param {import('$lib/instructor/bouncy_instructor').Challenge} challenge */
  function challengeAuthorHref(challenge) {
    const params = new URLSearchParams({
      videoUrl: challenge.solutionVideo.path(),
      beats: Array.from(challenge.solutionVideo.beats()).join(','),
      startTimestamp: String(challenge.solutionStartTimestamp),
    });
    return `/dev/challenge-author?${params}`;
  }

  onDestroy(() => {
    clearReview();
    unregisterTracker?.();
  });
</script>

{#if finished && detection}
  <ActivityReview
    {detection}
    videoUrl={reviewVideoSrc || ''}
    maximum={challenge.maxPoses}
    threshold={challenge.requiredHits / challenge.maxPoses}
    passed={succeeded}
    {recordingStart}
    {recordingEnd}
    onRestart={retry}
    onBack={() => goto(`/challenges/${set.id}`)}
  />
{:else}
  <StandardPage backButton title={challenge.name}>
    {#if !challenge}
      <h1>{$t('challenges.not-found')}</h1>
    {:else if !started}
      <p>{challenge.explanation || ''}</p>
      <p><strong>{challenge.bpm} {$t('challenges.bpm-unit')}</strong></p>
      <VideoWithMetaData
        video={challenge.solutionVideo}
        ariaLabel={`${challenge.name} ${$t('challenges.solution-video')}`}
      />
      <p>{$t('challenges.start-description')}</p>
      {#if availableSongs.length > 1}
        <label>
          {$t('challenges.song-label')}
          <select bind:value={selectedSongId}>
            {#each availableSongs as song}
              <option value={song.id}>{song.title} — {song.author}</option>
            {/each}
          </select>
        </label>
      {:else if availableSongs.length === 0}
        <p>{$t('challenges.no-compatible-song')}</p>
      {/if}
      <LoginRequiredContent>
        {#snippet guest({ apiUser })}
          <button onclick={() => start(apiUser)} disabled={!selectedSongId}>
            {$t('challenges.start-button')}
          </button>
        {/snippet}
      </LoginRequiredContent>
      {#if $dev}
        <button
          class="author-button"
          onclick={() => goto(challengeAuthorHref(challenge))}
          >{$t('challenges.edit-in-author-button')}</button
        >
      {/if}
    {:else if !finished}
      <div data-testid="challenge-live">
        <LiveActivity
          {onDone}
          songId={selectedSongId}
          {songTiming}
          teacherVideo={challenge.solutionVideo.path()}
          challengeDurationMs={challenge.challengeDurationMs}
          requiredHits={challenge.requiredHits}
        />
      </div>
    {:else}
      <p>
        {$t(
          invalidChallengeSong
            ? 'challenges.song-too-short'
            : succeeded
              ? savingCompletion
                ? 'challenges.completion-saving'
                : completionSaved
                  ? completionSyncPending
                    ? 'challenges.completion-sync-pending'
                    : 'challenges.completion-saved'
                  : 'challenges.completion-save-failed'
              : 'challenges.retry-description'
        )}
      </p>
      <p>{$t('challenges.review-unavailable')}</p>
    {/if}
  </StandardPage>
{/if}

<style>
  button,
  select {
    font: inherit;
    min-height: 44px;
  }
  button {
    padding: 0.6rem 1rem;
  }
  select {
    min-width: 44px;
  }
  .author-button {
    margin-left: 0.5rem;
  }
</style>
