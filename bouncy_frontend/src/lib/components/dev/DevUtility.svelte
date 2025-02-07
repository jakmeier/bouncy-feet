<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { DetectionState } from '$lib/instructor/bouncy_instructor_bg';
  import { getContext, onDestroy, onMount } from 'svelte';
  import { readable } from 'svelte/store';

  const trackerCtx = getContext('tracker');
  $: tracker = trackerCtx ? trackerCtx.tracker : null;

  $: state = tracker ? tracker.detectionState : readable(0);
  $: registerTracker(tracker);

  onDestroy(unregisterTracker);

  /**
   * @param { Tracker } t
   */
  function registerTracker(t) {
    window.nextTrackerState = () =>
      t.devSetState(
        Math.min(DetectionState.TrackingDone, $state + 1),
        performance.now()
      );
  }
  function unregisterTracker() {
    window.nextTrackerState = undefined;
  }
</script>
