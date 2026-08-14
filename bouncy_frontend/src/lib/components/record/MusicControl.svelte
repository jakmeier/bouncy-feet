<script>
  import { run } from 'svelte/legacy';

  import { DetectionState } from '$lib/instructor/bouncy_instructor';
  import { setBeatStart } from '$lib/stores/Beat';
  import { getContext, onDestroy } from 'svelte';

  /**
   * Music control component for LiveRecording, to keep that component reasonably sized.
   *
   * 1. Stop playing music when Positioning.
   * 2. Restart the playing track when Positioning is done.
   *
   */

  const tracker = getContext('tracker').tracker;
  let { detectionState } = tracker;
  let { resetTrack, resetTrackAt, stopTrack } = getContext('music');

  /** @type {{songTiming?: ResolvedSongTiming}} */
  let { songTiming } = $props();

  /** @type {DetectionState} */
  let prevDetectionState = DetectionState.Init;

  /** @param {DetectionState} newState */
  function newDetectionState(newState) {
    if (newState !== prevDetectionState) {
      if (newState === DetectionState.Positioning) {
        stopTrack();
      }
      if (
        newState === DetectionState.MusicLeadIn ||
        (newState === DetectionState.CountDown && !songTiming)
      ) {
        const target = tracker.countInEndTimestamp;
        if (target !== undefined) {
          setBeatStart(target);
          if (songTiming) {
            const playbackWallTime =
              target - songTiming.countInEndMs + songTiming.playbackStartMs;
            resetTrackAt(playbackWallTime, songTiming.playbackStartMs);
          } else {
            resetTrack();
          }
        } else {
          resetTrack();
        }
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
