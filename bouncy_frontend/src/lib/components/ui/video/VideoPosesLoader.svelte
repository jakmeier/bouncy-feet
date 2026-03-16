<script>
  import { Tracker } from '$lib/instructor/bouncy_instructor';
  import { registerTracker } from '$lib/stores/Beat';
  import { getContext, onMount } from 'svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import { PoseWrapper } from '$lib/instructor/bouncy_instructor';

  /**
   * @typedef {Object} Props
   * @property {ArrayBuffer} arrayBuffer
   * @property {number[]} timestampsMs
   * @property {number} width
   * @property {number} height
   * @property {number} height
   * @property {PoseWrapper[]} poses --bindable
   */

  /** @type {Props}*/
  let { arrayBuffer, timestampsMs, poses = $bindable([]) } = $props();
  let dataListener;

  let tracker = new Tracker();
  registerTracker(tracker);
  const poseCtx = getContext('pose');

  onMount(async () => {
    const blob = new Blob([arrayBuffer], { type: 'video/mp4' });
    const url = URL.createObjectURL(blob);
    const hiddenVideoElement = document.createElement('video');
    hiddenVideoElement.src = url;

    dataListener = await poseCtx.newPoseDetection(
      (
        /** @type {{ landmarks: string | any[]; }} */ result,
        /** @type {number | undefined} */ timestamp
      ) => {
        if (
          result.landmarks &&
          result.landmarks.length >= 1 &&
          timestamp !== undefined
        ) {
          const kp = landmarksToKeypoints(result.landmarks[0]);
          tracker.addKeypoints(kp, timestamp);
          const skeleton = tracker.skeletonWrapperAt(timestamp);
          const pose = skeleton?.pose();
          if (pose) {
            poses.push(pose);
          }
        }
      }
    );

    await hiddenVideoElement.play();
    hiddenVideoElement.pause();

    for (let t of timestampsMs) {
      await new Promise((resolve) => {
        hiddenVideoElement.currentTime = t / 1000;
        hiddenVideoElement.onseeked = resolve;
      });
      dataListener.trackFrame(hiddenVideoElement);
    }
  });
</script>
