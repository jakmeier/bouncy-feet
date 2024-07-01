<script>
  import Area from '$lib/components/ui/Area.svelte';
  import Camera from '$lib/components/record/Camera.svelte';
  import Canvas from '$lib/components/Canvas.svelte';
  import Avatar from '$lib/components/avatar/Avatar.svelte';
  import { hideNavigation } from '$lib/stores/UiState';
  import { getContext, onMount } from 'svelte';
  import { landmarksToKeypoints } from '$lib/pose';
  import BackgroundTask from '../BackgroundTask.svelte';
  import { writable } from 'svelte/store';

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
  const avatarSize = 140;
  const avatarLineWidth = 5;
  $: $hideNavigation = cameraOn;

  /** @type {Camera} */
  let camera;
  /** @type {HTMLVideoElement} */
  let cameraVideoElement;
  /** @type {import("$lib/instructor/bouncy_instructor").Skeleton | undefined} */
  let skeleton;
  /** @type {import("@mediapipe/tasks-vision").NormalizedLandmark[]} */
  let landmarks = [];
  /** @type {{ trackFrame: (arg0: HTMLVideoElement) => void; }} */
  let dataListener;

  /** @type {number} */
  const borderWidth = 5;
  /** @type {number} */
  let outerWidth;
  /** @type {number} */
  $: width = outerWidth - 2 * borderWidth;
  /** @type {number} */
  $: height = (width * 4) / 3;
  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcWidth = writable(100);
  /** @type {import('svelte/store').Writable<number>} */
  let videoSrcHeight = writable(150);

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

  const onPoseDetection = (
    /** @type {{ landmarks: import('@mediapipe/tasks-vision').Landmark[][]; }} */ result,
    /** @type {number} */ timestamp
  ) => {
    if (recordingStart === undefined) {
      recordingStart = timestamp;
    }
    if (result.landmarks && result.landmarks.length >= 1) {
      landmarks = result.landmarks[0];
      const kp = landmarksToKeypoints(result.landmarks[0]);
      const skeletons = tracker.addKeypoints(kp, timestamp);
      skeleton = skeletons.front;
      recordingEnd = timestamp;
    }
    // TODO(performance): do this less often
    onVideoResized();
  };

  function onVideoResized() {
    $videoSrcWidth = cameraVideoElement.clientWidth;
    $videoSrcHeight = cameraVideoElement.clientHeight;
  }

  onMount(async () => {
    dataListener = await poseCtx.newPoseDetection(onPoseDetection);
    onVideoResized();
  });
</script>

<div bind:clientWidth={outerWidth} style="width: 100%;">
  <BackgroundTask {onFrame}></BackgroundTask>

  <Area width="{width}px" height="{height}px" borderWidth="{borderWidth}px">
    <Camera
      {width}
      {height}
      bind:videoElement={cameraVideoElement}
      bind:cameraOn
      bind:this={camera}
    />
    <div
      class="avatar-container"
      style="left: {(width - $videoSrcWidth) / 2}px"
    >
      <Canvas width={$videoSrcWidth} height={$videoSrcHeight}>
        <Avatar
          skeleton={null}
          {landmarks}
          width={$videoSrcWidth}
          height={$videoSrcHeight}
          mainColor={'#382eebC0'}
          headColor={'#382eeb60'}
          secondColor={'#c2bfff40'}
        ></Avatar>
      </Canvas>
    </div>
  </Area>
</div>

<style>
  .avatar-container {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: 2;
  }
</style>
