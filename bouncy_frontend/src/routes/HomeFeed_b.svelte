<script>
  import { t } from '$lib/i18n.js';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter } from '$lib/stores/Beat';
  import HomeEntry from './HomeEntry.svelte';
  import PathwayProgress from './PathwayProgress.svelte';
  import Arrow from '$lib/components/ui/Arrow.svelte';
  import LightBackgroundSection from '$lib/components/ui/sections/LightSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import AvatarCustomizer from '$lib/components/avatar/AvatarCustomizer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  /**
   * @typedef {Object} Props
   * @property {any} featuredDances
   * @property {any} featuredSteps
   */

  /** @type {Props} */
  let { featuredDances, featuredSteps } = $props();

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

<LogoHeader />

<!-- WIP -->
<!-- <AvatarCustomizer /> -->

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

<LightBackgroundSection>
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
      <div class="centered small-space">
        <a href="https://github.com/jakmeier/bouncy-feet/issues">
          <button> GitHub </button>
        </a>
      </div>
    </div>
  </div>

  <div class="small-space">
    {$t('home.version-label')}:
    {$versionString}
  </div>

  <Footer />
</LightBackgroundSection>

<style>
  .transparent-box {
    border-radius: 10px;
    margin: 12rem 0;
    z-index: 1;
    position: relative;
    color: var(--theme-neutral-dark);
  }

  .centered {
    text-align: center;
  }
  .focus-card {
    max-width: 400px;
    padding: 5px 20px;
    min-height: 93dvh;
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

  .small-space {
    margin: 1rem 0px;
  }
  .space {
    margin: 3rem 0px;
  }

  .down-marker {
    max-width: 80px;
    max-height: 80px;
    rotate: 90deg;
    margin: 100px auto 0;
    padding: 10px 5px;
  }
</style>
