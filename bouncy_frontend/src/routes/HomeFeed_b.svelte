<script>
  import { t } from '$lib/i18n.js';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter } from '$lib/stores/Beat';
  import HomeEntry from './HomeEntry.svelte';
  import Arrow from '$lib/components/ui/Arrow.svelte';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import HelpUs from '$lib/components/info/HelpUs.svelte';
  import LifetimeStats from './profile/LifetimeStats.svelte';
  import About from '$lib/components/info/About.svelte';
  import Section from '$lib/components/ui/sections/Section.svelte';
  import BugReports from '$lib/components/info/BugReports.svelte';
  import Nerds from '$lib/components/info/Nerds.svelte';
  import Background from '$lib/components/ui/sections/Background.svelte';
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
   * @returns {import("bouncy_instructor").DanceWrapper}
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

<Background bgColor="var(--theme-neutral-black)" color="var(--theme-neutral-white)"></Background>
<LogoHeader />

<div class="focus-card">
  <h1>
    {$t('home.slogan-1')}
  </h1>
  <div>
    <HomeEntry dance={entryDance}></HomeEntry>
  </div>
  <div class="down-marker">
    <Arrow color="var(--theme-neutral-white)" />
  </div>
</div>

<Section bgColor={'var(--theme-main)'} color={'var(--theme-neutral-dark)'}>
  <LifetimeStats></LifetimeStats>

  <!-- TODO lesson for user picked coach -->
  <div class="section-end-button">
    <a href="./coach/chorok">
      <button>
        {$t('home.continue-learning-button')}
      </button>
    </a>
  </div>
</Section>

<About></About>
<Section bgColor={'var(--theme-accent)'} color={'var(--theme-neutral-dark)'}>
  <HelpUs />
</Section>

<Nerds />

<LightSection>
  <BugReports />
  <div class="small-space">
    {$t('home.version-label')}:
    {$versionString}
  </div>

  <Footer />
</LightSection>

<style>
  .focus-card {
    max-width: 400px;
    padding: 5px 20px;
    min-height: 93dvh;
    margin: auto;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .small-space {
    margin: 3rem 0px 1rem;
  }

  .down-marker {
    max-width: 80px;
    max-height: 80px;
    margin: 100px auto 0;
    padding: 10px 5px;
  }

  .section-end-button {
    margin: 4rem 0 1rem;
  }
</style>
