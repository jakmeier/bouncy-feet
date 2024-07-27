<script>
  import { counter } from '$lib/timer';
  import { getContext, onDestroy, onMount } from 'svelte';
  import Experience from './Experience.svelte';
  import DanceStats from '../../routes/profile/DanceStats.svelte';
  import Step from '../../routes/collection/Step.svelte';

  /** @type {DanceSessionResult?} */
  export let data;
  /** @type {import('$lib/instructor/bouncy_instructor').StepInfo} */
  export let step;

  const user = getContext('user').store;

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);

  /**
   * @type {{ slow: number; mid: number; fast: number; veryFast: number; }}
   */
  let stats;
  let averageBpm = 0;
  let gainedXp = 0;
  $: if (data) {
    stats = data.stats[step.name];
    averageBpm =
      data.bpms.reduce((acc, bpm) => acc + bpm, 0) / data.bpms.length;
    gainedXp = data.experience;
  }
</script>

<div class="step">
  <div class="exp">
    <Experience
      xp={$user.userSteps[step.name]
        ? $user.userSteps[step.name].experience
        : 0}
      height={50}
      twoRows
      lvlSize={150}
    ></Experience>
    <div>+{gainedXp} XP</div>
  </div>
  <Step {step} size={225} poseIndex={$i} {animationTime} />
</div>
{#if data}
  <DanceStats
    numSteps={data.numSteps}
    seconds={data.duration / 1000}
    slow={stats.slow}
    mid={stats.mid}
    fast={stats.fast}
    veryFast={stats.veryFast}
    {averageBpm}
  />
{/if}

<style>
  .step {
    display: grid;
  }
  .exp {
    margin: 35px 0 25px 0;
    text-align: center;
    font-size: 1.3em;
  }
</style>
