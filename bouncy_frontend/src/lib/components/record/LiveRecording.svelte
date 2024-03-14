<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Banner from '$lib/components/review/Banner.svelte';
  import Camera from '$lib/components/record/Camera.svelte';
  import SvgAvatar from '$lib/components/avatar/SvgAvatar.svelte';
  import Svg from '$lib/components/avatar/Svg.svelte';
  import { getContext, onDestroy, onMount } from 'svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import BackgroundTask from '../BackgroundTask.svelte';

  export let cameraOn = false;
  /** @type {undefined | number} */
  export let recordingStart;
  /** @type {undefined | number} */
  export let recordingEnd;
  /** @type {import("$lib/instructor/bouncy_instructor").DetectedStep[]} */
  export let detectedSteps = [];

  export const startCamera = async () => {
    await camera.startCamera();
  };
  export const startRecording = async () => {
    await camera.startRecording();
  };
  export const stopCamera = () => {
    camera.stopCamera();
  };
  export const endRecording = async () => {
    return await camera.endRecording();
  };

  const poseCtx = getContext('pose');
  let tracker = getContext('tracker').tracker;

  /** @type {Camera} */
  let camera;
  /** @type {HTMLVideoElement} */
  let cameraVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | undefined} */
  let skeleton;
  /** @type {{ trackFrame: (arg0: HTMLVideoElement) => void; }} */
  let dataListener;

  function onFrame() {
    if (cameraOn && dataListener) {
      const start = performance.now();
      dataListener.trackFrame(cameraVideoElement);
      const t = performance.now() - start;
      if (t > 50) {
        console.debug(`trackFrame took ${t}ms`);
      }
      detectedSteps = tracker.detectDance();
      const t2 = performance.now() - start;
      if (t2 - t > 30) {
        console.debug(`detectDance took ${t2 - t}ms`);
      }
    }
  }

  // Communicate a cursor seek from the video to the banner.
  /**
   * @type {(cursor: number) => void}
   */
  let setCursor;

  const onPoseDetection = (
    /** @type {{ landmarks: import('@mediapipe/tasks-vision').Landmark[][]; }} */ result,
    /** @type {number} */ timestamp
  ) => {
    if (recordingStart === undefined) {
      recordingStart = timestamp;
    }
    if (result.landmarks && result.landmarks.length >= 1) {
      const kp = landmarksToKeypoints(result.landmarks[0]);
      const skeletons = tracker.addKeypoints(kp, timestamp);
      skeleton = skeletons.front;
      recordingEnd = timestamp;
      setCursor(1.0);
    }
  };

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(onPoseDetection);
  });
</script>

<BackgroundTask {onFrame}></BackgroundTask>

<Area width="{282}px" height="{376}px">
  <Camera
    width={282}
    height={376}
    bind:videoElement={cameraVideoElement}
    bind:cameraOn
    bind:this={camera}
  />
</Area>

<Area width="{280}px" height="{280}px">
  <Svg height={280} width={280}>
    {#if skeleton}
      <SvgAvatar width={280} height={280} {skeleton} />
    {/if}
  </Svg>
</Area>

<Banner
  bind:setCursor
  steps={detectedSteps}
  reviewStart={recordingStart || 0}
  reviewEnd={recordingEnd || 1}
></Banner>
