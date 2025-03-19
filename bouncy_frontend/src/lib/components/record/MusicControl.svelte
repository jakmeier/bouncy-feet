<script>
  import { run } from 'svelte/legacy';

  import { DetectionState } from '$lib/instructor/bouncy_instructor';
  import { getContext, onDestroy } from 'svelte';

  /**
   * Music control component for LiveRecording, to keep that component reasonably sized.
   *
   * 1. Stop playing music when Positioning.
   * 2. Restart the playing track when Positioning is done.
   *
   */

  let { detectionState } = getContext('tracker').tracker;
  let { resetTrack, stopTrack } = getContext('music');

  /** @type {DetectionState} */
  let prevDetectionState = DetectionState.Init;

  /** @param {DetectionState} newState */
  function newDetectionState(newState) {
    if (newState !== prevDetectionState) {
      if (newState === DetectionState.Positioning) {
        stopTrack();
      }
      if (newState === DetectionState.CountDown) {
        resetTrack();
      }
      prevDetectionState = newState;
    }
  }
  run(() => {
    newDetectionState($detectionState);
  });

  onDestroy(() => {
    stopTrack();
  });
</script>
