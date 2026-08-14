<script>
  import { t } from '$lib/i18n.js';
  import { getContext, onMount } from 'svelte';
  import LiveRecording from '$lib/components/record/LiveRecording.svelte';
  import {
    DetectionResult,
    DetectionState,
    Tracker,
  } from '$lib/instructor/bouncy_instructor';
  import Popup from '$lib/components/ui/Popup.svelte';

  import Audio from '$lib/components/audio/BeatAudio.svelte';
  import { writable } from 'svelte/store';

  import { dev } from '$lib/stores/FeatureSelection';
  import DevUtility from '$lib/components/dev/DevUtility.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';

  /**
   * @typedef {Object} ChallengeAttemptOutcome
   * @property {boolean} succeeded
   * @property {number | undefined} requiredHits
   * @property {number} hits
   * @property {number} elapsedMs
   * @property {boolean} invalidChallengeSong
   */

  /**
   * @typedef {Object} Props
   * @property {(detection: DetectionResult, recordingStart: number, recordingEnd: number, videoUrl: string, outcome: ChallengeAttemptOutcome)=>void} [onDone]
   * @property {string} [teacherVideo]
   * @property {number} [challengeDurationMs]
   * @property {number} [requiredHits]
   * @property {string} [songId]
   * @property {ResolvedSongTiming} [songTiming]
   */

  /** @type {Props} */
  let {
    onDone,
    teacherVideo,
    challengeDurationMs,
    requiredHits,
    songId,
    songTiming,
  } = $props();

  let detectionResult;
  let videoUrl = $state('');

  /** @type {LiveRecording} */
  let liveRecording;
  /** @type {{ tracker: Tracker }} */
  const trackerContext = getContext('tracker');
  const { tracker } = trackerContext;
  const music = getContext('music');
  /** @type {import('svelte/store').Readable<DetectionState> | null} */
  let trackingState = tracker.detectionState;

  let useFixedBpm = false;
  /** @type {import('svelte/store').Writable<boolean>} */
  let startExercisePopUpIsOpen = writable(false);

  let showHintBeforeStart = true;

  export function clear() {
    videoUrl = '';
    tracker.clear();
    trackingState = tracker.detectionState;
  }

  async function start() {
    if (songId) await music.prepareTrack(songId);
    if (liveRecording) {
      await liveRecording.startCamera();
    }
    if (showHintBeforeStart) {
      // only show the hint once per sessions
      showHintBeforeStart = false;
      $startExercisePopUpIsOpen = true;
      return;
    }
    if (liveRecording) {
      await liveRecording.startRecording();
    }
  }

  /**
   * @param {Blob | null | undefined} videoBlob
   * @param {number} recordingStart
   * @param {number} recordingEnd
   * @param {ChallengeAttemptOutcome} outcome
   */
  async function onRecordingStopped(
    videoBlob,
    recordingStart,
    recordingEnd,
    outcome
  ) {
    tracker?.finishTracking();
    if (videoUrl) {
      URL.revokeObjectURL(videoUrl);
    }
    if (videoBlob) {
      videoUrl = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
      videoUrl = '';
    }

    detectionResult = tracker?.lastDetection;
    if (onDone) {
      onDone(detectionResult, recordingStart, recordingEnd, videoUrl, outcome);
    }
  }

  function closeStartExercisePopUp() {
    $startExercisePopUpIsOpen = false;
    start();
  }

  onMount(start);
</script>

<LightBackground />

<div class="outer">
  <LiveRecording
    bind:this={liveRecording}
    onStop={onRecordingStopped}
    forceBeat={true}
    {teacherVideo}
    {songTiming}
    {challengeDurationMs}
    {requiredHits}
  ></LiveRecording>
</div>

<Audio></Audio>

<Popup showOkButton title={'common.hint-popup-title'}>
  {$t('record.estimate-bpm-hint')}
</Popup>

{#if $dev}
  <DevUtility />
{/if}

<style>
  .outer {
    text-align: center;
    min-height: 100dvh;
  }
</style>
