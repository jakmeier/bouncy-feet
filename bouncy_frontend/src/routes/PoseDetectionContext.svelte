<script>
  import { PoseDetection } from '$lib/pose';
  /**
   * PoseDetectionContext component
   *
   * Provides access to a PoseDetection factory and manages the cleaning up of
   * resources related to it.
   * 
   * This is about raw media pipe poses, not bouncy feet poses.
   */
  import { setContext, onDestroy, getContext } from 'svelte';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

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

{@render children?.()}
