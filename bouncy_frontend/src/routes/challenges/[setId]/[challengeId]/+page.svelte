<script>
  import { getContext, onDestroy } from 'svelte';
  import { goto } from '$app/navigation';
  import { dev } from '$lib/stores/FeatureSelection';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import LiveActivity from '$lib/components/activity/LiveActivity.svelte';
  import VideoReview from '$lib/components/review/VideoReview.svelte';
  import { registerTracker, setBpm } from '$lib/stores/Beat';
  import { completeChallenge } from '$lib/challengeProgress.js';
  import { t } from '$lib/i18n';

  let { data } = $props();
  const { getSet } = getContext('challengeSets');
  const user = getContext('user');
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
  let finished = $state(false);
  let succeeded = $state(false);
  /** @type {import('$lib/instructor/bouncy_instructor').DetectionResult | undefined} */
  let detection = $state();
  /** @type {number | undefined} */
  let recordingStart = $state();
  /** @type {number | undefined} */
  let recordingEnd = $state();
  /** @type {string | undefined} */
  let reviewVideoSrc = $state();
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

  /**
   * @param {import('$lib/instructor/bouncy_instructor').DetectionResult} newDetection
   * @param {number} newRecordingStart
   * @param {number} newRecordingEnd
   * @param {string} newVideoUrl
   */
  async function onDone(
    newDetection,
    newRecordingStart,
    newRecordingEnd,
    newVideoUrl
  ) {
    detection = newDetection;
    recordingStart = newRecordingStart;
    recordingEnd = newRecordingEnd;
    reviewVideoSrc = newVideoUrl;
    finished = true;

    // `poseMatches` counts poses, whereas `trackedSubbeats` counts timing
    // intervals. Compare matches with the tracker-provided pose count instead.
    succeeded =
      Boolean(detection) &&
      detection.poseMisses === 0 &&
      detection.poseMatches >= tracker.expectedPoses;
    if (succeeded && user.apiUser) {
      await completeChallenge(user.apiUser, set, challenge.id);
      user.apiUser.submitActivity(
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

<StandardPage backButton title={challenge.name}>
  {#if !challenge}
    <h1>{$t('challenges.not-found')}</h1>
  {:else if !started}
    <p>{challenge.explanation || ''}</p>
    <p><strong>{challenge.bpm} BPM</strong></p>
    <!-- svelte-ignore a11y_media_has_caption -->
    <video
      controls
      src={challenge.solutionVideo.path()}
      aria-label={`${challenge.name} ${$t('challenges.solution-video')}`}
    ></video>
    <p>{$t('challenges.start-description')}</p>
    <button onclick={start}>{$t('challenges.start-button')}</button>
    {#if $dev}
      <button
        class="author-button"
        onclick={() => goto(challengeAuthorHref(challenge))}
        >{$t('challenges.edit-in-author-button')}</button
      >
    {/if}
  {:else if !finished}
    <div data-testid="challenge-live">
      <LiveActivity {onDone} teacherVideo={challenge.solutionVideo.path()} />
    </div>
  {:else}
    <h1>
      {$t(succeeded ? 'challenges.completed-title' : 'challenges.not-completed-title')}
    </h1>
    <p>
      {$t(succeeded ? 'challenges.completion-saved' : 'challenges.retry-description')}
    </p>
    {#if detection && recordingStart !== undefined && recordingEnd !== undefined}
      <VideoReview
        reviewVideoSrc={reviewVideoSrc || ''}
        detectedSteps={detection.steps()}
        reviewHint="challenges.review-hint"
        {recordingStart}
        {recordingEnd}
      />
    {:else}
      <p>{$t('challenges.review-unavailable')}</p>
    {/if}
    <button onclick={retry}>{$t('challenges.retry-button')}</button>
    <a href={`/challenges/${set.id}`}>{$t('challenges.back-button')}</a>
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
  .author-button {
    margin-left: 0.5rem;
  }
</style>
