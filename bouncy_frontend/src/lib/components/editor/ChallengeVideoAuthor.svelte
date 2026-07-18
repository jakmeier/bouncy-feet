<script>
  import { onDestroy } from 'svelte';
  import { PoseFileWrapper, Tracker } from '$lib/instructor/bouncy_instructor';
  import { PoseDetection, landmarksToKeypoints } from '$lib/pose';
  import { fileToUrl } from '$lib/promise_util';
  import { downloadTextFile } from '$lib/text_utils';
  import PoseAnglesForm from './PoseAnglesForm.svelte';
  import PoseWeightsForm from './PoseWeightsForm.svelte';
  import Svg from '../avatar/Svg.svelte';
  import SvgAvatar2 from '../avatar/SvgAvatar2.svelte';

  const defaultVideo =
    'https://app.bouncy-feet.ch/media/videos/challenges/jumping_jack.mp4';

  let videoUrl = $state(defaultVideo);
  let video = $state();
  let videoWidth = $state(0);
  let videoHeight = $state(0);
  let durationMs = $state(0);
  let timestampMs = $state(0);
  let skeleton = $state();
  /** @type {PoseDetection | undefined} */
  let detector;
  let tracker = new Tracker();
  /** @type {any} */
  let anglesForm;
  /** @type {any} */
  let weightsForm;
  let poseId = $state('jumping-jack-open');
  let stepId = $state('jumping-jack');
  let stepName = $state('Jumping jack');
  let jumpHeight = $state('1.0');
  /** @type {number[]} */
  let beats = $state([]);
  let startTimestamp = $state(0);
  /** @type {{id: string, pose: import('bouncy_instructor').PoseWrapper}[]} */
  let captured = $state([]);
  /** @type {{id: string, jumpHeight: string}[]} */
  let keyframes = $state([]);
  let message = $state('');
  let request = 0;

  onDestroy(() => detector?.close());

  function videoTime() {
    return Math.round((video?.currentTime || 0) * 1000);
  }

  async function loadUrl() {
    if (!video) return;
    video.src = videoUrl;
    video.load();
  }

  /** @param {Event} event */
  async function loadFile(event) {
    const file = /** @type {HTMLInputElement} */ (event.currentTarget)
      .files?.[0];
    if (!file || !video) return;
    videoUrl = await fileToUrl(file);
    await loadUrl();
  }

  function metadataLoaded() {
    durationMs = Math.round(video.duration * 1000);
    timestampMs = videoTime();
    inspectFrame();
  }

  /** @param {number} ms */
  function seekTo(ms) {
    if (!video || !Number.isFinite(video.duration)) return;
    video.currentTime = Math.max(0, Math.min(ms, durationMs)) / 1000;
  }

  function seeked() {
    timestampMs = videoTime();
    inspectFrame();
  }

  // A fresh detector is intentional: MediaPipe VIDEO timestamps must be monotonic,
  // while an author needs to scrub backwards freely.
  async function inspectFrame() {
    if (!video || video.readyState < HTMLMediaElement.HAVE_CURRENT_DATA) return;
    const currentRequest = ++request;
    detector?.close();
    tracker = new Tracker();
    skeleton = undefined;
    const frameTimestamp = videoTime();
    detector = await PoseDetection.new((result) => {
      if (currentRequest !== request || !result.landmarks?.[0]) return;
      const keypoints = landmarksToKeypoints(result.landmarks[0]);
      tracker.addKeypoints(keypoints, frameTimestamp);
      skeleton = tracker.renderedKeypointsAt(
        frameTimestamp,
        videoWidth,
        videoHeight
      );
    }, frameTimestamp);
    if (currentRequest === request) detector.trackFrame(video, frameTimestamp);
  }

  function capturePose() {
    const poseSkeleton = tracker.skeletonWrapperAt(timestampMs);
    if (!poseSkeleton) {
      message = 'No person was detected in this frame.';
      return;
    }
    anglesForm.loadSkeleton(poseSkeleton);
    weightsForm.loadPose(anglesForm.readPose());
    message = `Loaded frame ${timestampMs} ms. Edit the pose, then save it.`;
  }

  function savePose() {
    const pose = weightsForm?.getPose();
    const id = poseId.trim();
    if (!pose || !id) {
      message = 'Capture a frame and provide a pose ID first.';
      return;
    }
    pose.setId(id);
    const existing = captured.findIndex((entry) => entry.id === id);
    const entry = { id, pose };
    if (existing >= 0) captured[existing] = entry;
    else captured.push(entry);
    captured = captured;
    message = `Saved ${id}.`;
  }

  /** @param {string} id */
  function addKeyframe(id) {
    if (!captured.some((entry) => entry.id === id)) return;
    keyframes = [...keyframes, { id, jumpHeight }];
  }

  function addBeat() {
    beats = [...new Set([...beats, timestampMs])].sort((a, b) => a - b);
  }

  /** @param {number} beat */
  function removeBeat(beat) {
    beats = beats.filter((value) => value !== beat);
  }

  function addStart() {
    startTimestamp = timestampMs;
    if (!beats.includes(timestampMs)) addBeat();
  }

  function poseRon() {
    const file = new PoseFileWrapper();
    captured.forEach(({ pose }) => file.addPose(pose));
    return file.buildPrettyRon();
  }

  function challengePosesRon() {
    // PoseFileWrapper writes a complete `(version, poses)` file. A challenge
    // already owns its version field, so copy only its `poses: [...]` field.
    return poseRon()
      .replace(/^\(\s*\n?\s*version:\s*\d+,\s*\n?/, '')
      .replace(/\n?\s*\)\s*$/, '')
      .trim();
  }

  function stepRon() {
    const frames = keyframes
      .map(({ id, jumpHeight: height }) =>
        height === '' || Number(height) === 0
          ? `        (pose: ${JSON.stringify(id)}, orientation: ToCamera),`
          : `        (pose: ${JSON.stringify(id)}, orientation: ToCamera, jump_height: ${Number(height)}),`
      )
      .join('\n');
    return `steps: [\n  (\n    name: ${JSON.stringify(stepName)},\n    id: ${JSON.stringify(stepId)},\n    keyframes: [\n${frames}\n    ],\n  ),\n],`;
  }

  function videoRon() {
    const marker = startTimestamp;
    return `solution_video: (\n  path: ${JSON.stringify(videoUrl)},\n  beats: [${beats.join(', ')}],\n  markers: {${marker}: Start},\n),\nsolution_start_timestamp: ${marker},`;
  }

  /** @param {string} text @param {string} label */
  async function copy(text, label) {
    await navigator.clipboard.writeText(text);
    message = `${label} copied to clipboard.`;
  }
</script>

<div class="author">
  <p>
    Dev-only helper. Capture poses and copy the resulting RON into
    <code>src/lib/assets/challenges/jumping-jacks.ron</code>.
  </p>

  <div class="source">
    <input aria-label="Video URL" bind:value={videoUrl} />
    <button onclick={loadUrl}>Load URL</button>
    <input
      aria-label="Local video"
      type="file"
      accept="video/*"
      onchange={loadFile}
    />
  </div>

  <!-- svelte-ignore a11y_media_has_caption -->
  <div class="preview">
    <video
      bind:this={video}
      bind:videoWidth
      bind:videoHeight
      crossorigin="anonymous"
      controls
      playsinline
      onloadedmetadata={metadataLoaded}
      onseeked={seeked}
    ></video>
    {#if skeleton}
      <Svg width={videoWidth} height={videoHeight} orderByZ showOverflow>
        <SvgAvatar2 {skeleton} />
      </Svg>
    {/if}
  </div>

  <div class="timeline">
    <button onclick={() => seekTo(timestampMs - 100)}>−100 ms</button>
    <button onclick={() => seekTo(timestampMs - 1000 / 30)}>−frame</button>
    <input
      type="range"
      min="0"
      max={durationMs || 1}
      step="1"
      value={timestampMs}
      oninput={(event) => seekTo(Number(event.currentTarget.value))}
    />
    <button onclick={() => seekTo(timestampMs + 1000 / 30)}>+frame</button>
    <button onclick={() => seekTo(timestampMs + 100)}>+100 ms</button>
    <output>{timestampMs} ms</output>
  </div>

  <section>
    <h2>Solution-video beats</h2>
    <button onclick={addBeat}>Add beat at {timestampMs} ms</button>
    <button onclick={addStart}>Set Start marker here</button>
    <div class="chips">
      {#each beats as beat}
        <button
          class:start={beat === startTimestamp}
          onclick={() => seekTo(beat)}
          >{beat} ms {beat === startTimestamp ? '(Start)' : ''} ×</button
        >
        <button
          aria-label={`Remove ${beat} ms`}
          onclick={() => removeBeat(beat)}>×</button
        >
      {/each}
    </div>
  </section>

  <section>
    <h2>Pose at selected frame</h2>
    <button onclick={capturePose}>Capture selected frame</button>
    <label>Pose ID <input bind:value={poseId} /></label>
    <PoseAnglesForm bind:this={anglesForm} />
    <PoseWeightsForm bind:this={weightsForm} />
    <button onclick={savePose}>Save staged pose</button>
    {#if captured.length}
      <div class="chips">
        {#each captured as entry}
          <button onclick={() => addKeyframe(entry.id)}
            >Add {entry.id} as keyframe</button
          >
        {/each}
      </div>
    {/if}
  </section>

  <section>
    <h2>Step</h2>
    <label>ID <input bind:value={stepId} /></label>
    <label>Name <input bind:value={stepName} /></label>
    <label
      >Jump height for next keyframe <input
        type="number"
        step="0.1"
        bind:value={jumpHeight}
      /></label
    >
    <ol>
      {#each keyframes as frame, index}
        <li>
          {frame.id}, jump height {frame.jumpHeight}
          <button
            onclick={() =>
              (keyframes = keyframes.filter((_, i) => i !== index))}
            >Remove</button
          >
        </li>
      {/each}
    </ol>
  </section>

  <section class="exports">
    <button onclick={() => copy(videoRon(), 'Video alignment')}
      >Copy video alignment RON</button
    >
    <button onclick={() => copy(stepRon(), 'Step')}>Copy step RON</button>
    <button onclick={() => copy(challengePosesRon(), 'Challenge poses')}
      >Copy challenge poses RON</button
    >
    <button
      onclick={() => downloadTextFile('jumping-jacks.poses.ron', poseRon())}
      >Download pose file</button
    >
  </section>
  {#if message}<p class="message">{message}</p>{/if}
</div>

<style>
  .author {
    max-width: 1000px;
    margin: 1rem auto 4rem;
  }
  .source,
  .timeline,
  .exports,
  label,
  .chips {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-wrap: wrap;
    margin: 0.75rem 0;
  }
  .source input:first-child {
    flex: 1;
    min-width: 18rem;
  }
  .preview {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.5rem;
    align-items: start;
  }
  video {
    width: 100%;
    max-height: 50vh;
    background: black;
  }
  .timeline input {
    flex: 1;
    min-width: 12rem;
  }
  section {
    border-top: 1px solid var(--theme-neutral-gray);
    margin-top: 1rem;
    padding-top: 0.5rem;
  }
  .chips button.start {
    font-weight: bold;
  }
  .message {
    font-weight: bold;
  }
  @media (max-width: 650px) {
    .preview {
      grid-template-columns: 1fr;
    }
  }
</style>
