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

  import Audio from '$lib/components/BeatAudio.svelte';
  import { writable } from 'svelte/store';

  import { dev } from '$lib/stores/FeatureSelection';
  import DevUtility from '$lib/components/dev/DevUtility.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  /**
   * @typedef {Object} Props
   * @property {(detection: DetectionResult, recordingStart: number, recordingEnd: number, videoUrl: string)=>void} [onDone]
   * @property {number} videoOpacity
   * @property {boolean} enableLiveAvatar
   * @property {boolean} enableInstructorAvatar
   */

  /** @type {Props} */
  let { onDone, videoOpacity, enableLiveAvatar, enableInstructorAvatar } =
    $props();

  let detectionResult;
  /** @type {string} */
  let videoUrl = $state();

  let liveRecording;
  /** @type {Tracker | undefined} */
  let { tracker } = getContext('tracker');
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
   * @param {Blob | undefined} videoBlob
   */
  async function onRecordingStopped(videoBlob, recordingStart, recordingEnd) {
    tracker?.finishTracking();
    if (videoBlob) {
      if (videoUrl) {
        URL.revokeObjectURL(videoUrl);
      }
      videoUrl = URL.createObjectURL(videoBlob);
    } else {
      console.warn('ended recording and did not get video blob', videoBlob);
    }

    detectionResult = tracker?.lastDetection;
    // TODO: consider storing stats here
    // const sessionResult = computeDanceStats(detected.steps());
    // if (sessionResult) {
    //   addDanceToStats(sessionResult);
    // }
    if (onDone) {
      onDone(detectionResult, recordingStart, recordingEnd, videoUrl);
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
    {videoOpacity}
    {enableLiveAvatar}
    {enableInstructorAvatar}
    forceBeat
  ></LiveRecording>
</div>

<Audio isOn={useFixedBpm && $trackingState !== DetectionState.TrackingDone}
></Audio>

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
