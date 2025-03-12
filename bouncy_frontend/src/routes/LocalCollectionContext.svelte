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
    loadLocalSteps,
    addLocalPoses,
    DanceWrapper,
  } from '$lib/instructor/bouncy_instructor';
  import { setContext } from 'svelte';
  import { derived, writable } from 'svelte/store';
  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   */

  /** @type {Props} */
  let { children } = $props();

  // handle corrupt local storage etc without the app crashing
  let corruptDances = false;
  /**
   * @param {string} ron
   * @param {{ fromRon: ((ron: string) => any) }} wrapperConstructor
   */
  function safeFromRon(ron, wrapperConstructor) {
    try {
      return wrapperConstructor.fromRon(ron);
    } catch (e) {
      alert(`Error loading data: ${e}`);
      return null;
    }
  }

  let lockLocalStorage = false;

  const poseRon = browser ? localStorage.poses : null;
  let poseFileBuilder = poseRon
    ? safeFromRon(poseRon, PoseFileWrapper)
    : new PoseFileWrapper();
  if (poseFileBuilder === null) {
    // local storage could not be parsed
    // overwriting it might lost user data, let's avoid that
    lockLocalStorage = true;
    // but let's keep the app functional
    poseFileBuilder = new PoseFileWrapper();
  }
  const poseBuilderStore = writable(poseFileBuilder);
  addLocalPoses(poseFileBuilder.poses());

  const stepRon = browser ? localStorage.steps : null;
  let stepFileBuilder = stepRon
    ? safeFromRon(stepRon, StepFileWrapper)
    : new StepFileWrapper();
  if (stepFileBuilder === null) {
    // local storage could not be parsed
    // overwriting it might lost user data, let's avoid that
    lockLocalStorage = true;
    // but let's keep the app functional
    stepFileBuilder = new StepFileWrapper();
  }
  const stepBuilderStore = writable(stepFileBuilder);
  loadLocalSteps(stepFileBuilder.steps());

  const danceRon = browser ? localStorage.dances : null;
  let danceFileBuilder = danceRon
    ? safeFromRon(danceRon, DanceFileBuilder)
    : new DanceFileBuilder();
  if (danceFileBuilder === null) {
    // local storage could not be parsed
    // overwriting it might lost user data, let's avoid that
    lockLocalStorage = true;
    // but let's keep the app functional
    danceFileBuilder = new DanceFileBuilder();
  }
  const danceBuilderStore = writable(danceFileBuilder);

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
    addStep,
    removeStep,
    addPose,
    removePose,
  };

  if (browser) {
    ctx.poseBuilder.subscribe((/** @type {PoseFileWrapper} */ builder) => {
      if (!lockLocalStorage) {
        localStorage.poses = builder.buildRon();
      }
      // TODO: allow deleting poses
      addLocalPoses(builder.poses());
    });
    ctx.stepBuilder.subscribe((/** @type {StepFileWrapper} */ builder) => {
      if (!lockLocalStorage) {
        localStorage.steps = builder.buildRon();
      }
      loadLocalSteps(builder.steps());
    });
    ctx.danceBuilder.subscribe((/** @type {DanceFileBuilder} */ builder) => {
      if (!lockLocalStorage) {
        localStorage.dances = builder.buildRon();
      }
    });
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
      $stepBuilderStore.overwriteStep(step);
    } catch {
      $stepBuilderStore.addStep(step);
    }
    // trigger update (can I do better?)
    $stepBuilderStore = $stepBuilderStore;
  }

  /**
   * @param {String} id
   */
  function removeStep(id) {
    // before deleting, we must check that no dance uses the step!
    /** @type {DanceWrapper[]} */
    const dances = $danceBuilderStore.dances();
    for (let dance of dances) {
      const steps = dance.steps();
      for (let step of steps) {
        if (step.id === id) {
          throw new Error('Step in use');
        }
      }
    }

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
    // before deleting, we must check that no step uses the pose!
    /** @type {StepWrapper[]} */
    const steps = $stepBuilderStore.steps();
    for (let step of steps) {
      const poses = step.poses();
      for (let pose of poses) {
        if (pose.id() === id) {
          throw new Error('Pose in use');
        }
      }
    }

    $poseBuilderStore.removePose(id);
    // trigger update (can I do better?)
    $poseBuilderStore = $poseBuilderStore;
  }
</script>

{@render children?.()}
