<script>
  import { t } from '$lib/i18n.js';
  import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from '$lib/constants';
  import { base } from '$app/paths';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter } from '$lib/stores/Beat';
  import HomeEntry from './HomeEntry.svelte';
  import PathwayProgress from './PathwayProgress.svelte';
  import Arrow from '$lib/components/ui/Arrow.svelte';

  /** @type{import("$lib/instructor/bouncy_instructor").DanceWrapper[]} */
  export let featuredDances;
  /** @type{import("$lib/instructor/bouncy_instructor").StepWrapper[]} */
  export let featuredSteps;

  const entryDance = featuredDances.find(
    (dance) => dance.id === 'Home Animation (dev)'
  );

  /**
   * @param {number} i
   * @returns {import("$lib/instructor/bouncy_instructor").DanceWrapper}
   */
  function dance(i) {
    return featuredDances[i % featuredDances.length];
  }

  let animationDelay = 99;
  beatCounter.subscribe((counter) => {
    if (counter > 0) {
      animationDelay = 0;
    }
  });
</script>

<div class="title">
  <img
    class="logo"
    src="{base}/icons/icon_on_black.svg"
    alt="Bouncy Feet Logo"
  />
</div>

<div class="focus-card">
  <h1>
    {$t('home.test0')}
  </h1>
  <div>
    <HomeEntry dance={entryDance}></HomeEntry>
  </div>
  <div class="down-marker">
    <Arrow color="var(--theme-neutral-white)" />
  </div>
</div>

<div class="light-background">
  <div class="space">
    <div class="transparent-box">
      <h2>
        {$t('home.progress-title')}
      </h2>
    </div>
  </div>

  <div class="transparent-box">
    <PathwayProgress
      teacherName="V-Step Master"
      step={featuredSteps[1]}
      style={LEFT_RIGHT_COLORING}
      experience={700}
      skill={7}
      maxSkill={10}
      totalSteps={1703}
    />
  </div>

  <div class="transparent-box">
    <PathwayProgress
      teacherName="Running Man Coach"
      step={featuredSteps[0]}
      style={ORANGE_COLORING}
      experience={0}
      skill={0}
      maxSkill={10}
      totalSteps={0}
    />
  </div>

  <div class="space">
    <div class="transparent-box">
      <div>
        {$t('home.go-to-github')}
      </div>
      <div class="centered">
        <a href="https://github.com/jakmeier/bouncy-feet/issues">
          <button class="light"> GitHub </button>
        </a>
      </div>
    </div>
  </div>

  <div class="transparent-box">
    <div>
      <i>
        {$t('home.version-label')}:
        {$versionString}
      </i>
    </div>
  </div>

  <div class="brand-footer">
    <img
      class="logo rotated"
      src="{base}/icons/bouncyfeet.svg"
      alt="Bouncy Feet Logo"
    />
  </div>
</div>

<style>
  .title {
    position: relative;
    padding: 0 0 5px;
    background-color: var(--theme-neutral-black);
    color: var(--theme-neutral-white);
    z-index: 1;
    height: 8vh;
  }
  .title img.logo {
    height: 100%;
    /* Go outside the usual borders */
    margin: -10px;
  }

  .light-background {
    padding: 10px;
    margin: -10px;
    background-color: var(--theme-neutral-light);
    color: var(--theme-neutral-black);
  }

  .transparent-box {
    padding: 20px;
    border-radius: 10px;
    margin: 15px 5px;
    z-index: 1;
    position: relative;
    text-align: center;
    color: var(--theme-main-dark);
  }

  .centered {
    margin-top: 15px;
    text-align: center;
  }
  .brand-footer img {
    max-height: min(10vw, 50px);
    max-width: unset;
    margin: 10px 0;
  }
  .focus-card {
    max-width: 400px;
    padding: 5px 20px;
    min-height: 92vh;
    margin: auto;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }
  button {
    margin: 5px;
    width: fit-content;
    height: fit-content;
  }

  .space {
    margin: 60px 10px;
  }

  .down-marker {
    max-width: 80px;
    max-height: 80px;
    rotate: 90deg;
    margin: 100px auto 0;
    padding: 10px 5px;
  }
</style>
