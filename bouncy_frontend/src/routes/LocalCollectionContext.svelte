<script>
  /**
   * Provides access to locally stored content.
   */
  import { browser } from '$app/environment';
  import {
    DanceFileBuilder,
    DanceBuilder,
    PoseFileWrapper,
    PoseWrapper,
    StepWrapper,
    StepFileWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import { setContext } from 'svelte';
  import { derived, writable } from 'svelte/store';

  const danceRon = browser ? localStorage.dances : null;
  const danceFileBuilder = danceRon
    ? DanceFileBuilder.fromRon(danceRon)
    : new DanceFileBuilder();
  const danceBuilderStore = writable(danceFileBuilder);

  const stepRon = browser ? localStorage.steps : null;
  const stepFileBuilder = stepRon
    ? StepFileWrapper.fromRon(stepRon)
    : new StepFileWrapper();
  const stepBuilderStore = writable(stepFileBuilder);

  const poseRon = browser ? localStorage.poses : null;
  const poseFileBuilder = poseRon
    ? PoseFileWrapper.fromRon(poseRon)
    : new PoseFileWrapper();
  const poseBuilderStore = writable(poseFileBuilder);

  const ctx = {
    danceBuilder: danceBuilderStore,
    dances: derived(danceBuilderStore, ($b) => $b.dances()),
    stepBuilder: stepBuilderStore,
    steps: derived(stepBuilderStore, ($b) => $b.steps()),
    poseBuilder: poseBuilderStore,
    poses: derived(poseBuilderStore, ($b) => $b.poses()),
    addDanceBuilder,
    overwriteDanceBuilder,
    removeDance,
    addPose,
    removePose,
  };

  if (browser) {
    ctx.danceBuilder.subscribe(
      (/** @type {DanceFileBuilder} */ builder) =>
        (localStorage.dances = builder.buildRon())
    );
    ctx.stepBuilder.subscribe(
      (/** @type {StepFileWrapper} */ builder) =>
        (localStorage.steps = builder.buildRon())
    );
    ctx.poseBuilder.subscribe(
      (/** @type {PoseFileWrapper} */ builder) =>
        (localStorage.poses = builder.buildRon())
    );
  }

  setContext('localCollection', ctx);

  /**
   * @param {DanceBuilder} danceBuilder
   */
  function addDanceBuilder(danceBuilder) {
    $danceBuilderStore.addDance(danceBuilder);
    // trigger update (can I do better?)
    $danceBuilderStore = $danceBuilderStore;
  }

  /**
   * @param {DanceBuilder} danceBuilder
   */
  function overwriteDanceBuilder(danceBuilder) {
    $danceBuilderStore.overwriteDance(danceBuilder);
    // trigger update (can I do better?)
    $danceBuilderStore = $danceBuilderStore;
  }

  /**
   * @param {String} id
   */
  function removeDance(id) {
    $danceBuilderStore.removeDance(id);
    // trigger update (can I do better?)
    $danceBuilderStore = $danceBuilderStore;
  }

  /**
   * @param {StepWrapper} step
   */
  function addStep(step) {
    try {
      $stepBuilderStore.overwritePose(step);
    } catch {
      $stepBuilderStore.addPose(step);
    }
    // trigger update (can I do better?)
    $stepBuilderStore = $stepBuilderStore;
  }

  /**
   * @param {String} id
   */
  function removeStep(id) {
    $stepBuilderStore.removeStep(id);
    // trigger update (can I do better?)
    $stepBuilderStore = $stepBuilderStore;
  }

  /**
   * @param {PoseWrapper} pose
   */
  function addPose(pose) {
    try {
      $poseBuilderStore.overwritePose(pose);
    } catch {
      $poseBuilderStore.addPose(pose);
    }
    // trigger update (can I do better?)
    $poseBuilderStore = $poseBuilderStore;
  }

  /**
   * @param {String} id
   */
  function removePose(id) {
    $poseBuilderStore.removePose(id);
    // trigger update (can I do better?)
    $poseBuilderStore = $poseBuilderStore;
  }
</script>

<slot />
