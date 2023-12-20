<script>
  import { PoseDetection } from '$lib/pose';
  /**
   * PoseDetectionContext component
   *
   * Provides access to a PoseDetection factory and manages the cleaning up of
   * resources related to it.
   */
  import { setContext, onDestroy, getContext } from 'svelte';

  /**
   * @type {PoseDetection[]}
   */
  let detectors = [];

  let newPoseDetection = async function (
    /** @type {(result: import("@mediapipe/tasks-vision").PoseLandmarkerResult, timestamp: number) => void} */
    consumer
  ) {
    const newDetector = await PoseDetection.new(consumer);
    detectors.push(newDetector);
    return newDetector;
  };

  onDestroy(() => {
    detectors.forEach((d) => d.close());
    detectors = [];
  });

  setContext('pose', {
    newPoseDetection,
  });
</script>

<slot />
