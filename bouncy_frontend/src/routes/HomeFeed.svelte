<script>
  import { t } from '$lib/i18n.js';
  import { versionString } from '$lib/stores/FeatureSelection';
  import { beatCounter } from '$lib/stores/Beat';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import HelpUs from '$lib/components/info/HelpUs.svelte';
  import LifetimeStats from './profile/LifetimeStats.svelte';
  import About from '$lib/components/info/About.svelte';
  import Section from '$lib/components/ui/sections/Section.svelte';
  import BugReports from '$lib/components/info/BugReports.svelte';
  import Nerds from '$lib/components/info/Nerds.svelte';
  import Background from '$lib/components/ui/sections/Background.svelte';
  import { getContext } from 'svelte';
  import Personalities from './Personalities.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import HomeNote from './HomeNote.svelte';
  import { goto } from '$app/navigation';
  import { asset } from '$app/paths';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import UserList from '$lib/components/user/UserList.svelte';
  /**
   * @typedef {Object} Props
   * @property {any} featuredDances
   * @property {any} featuredSteps
   */

  /** @type {Props} */
  let {} = $props();

  /** @type {LocalState}*/
  const localState = getContext('localState');

  let animationDelay = 99;
  beatCounter.subscribe((counter) => {
    if (counter > 0) {
      animationDelay = 0;
    }
  });
  let imageHeight = $state(100);
</script>

<Background
  bgColor="var(--theme-neutral-black)"
  color="var(--theme-neutral-white)"
></Background>
<LogoHeader button="account_circle" onAction={() => goto('/profile')} />

<HomeNote />

<!-- <LogoHeader title={$t('home.slogan-1')} /> -->

<!-- TODO: maybe showcase something here -->

<DarkSection arrow>
  <Personalities></Personalities>
</DarkSection>

<LimeSection>
  <h1>{$t('home.users-title')}</h1>
  <UserList></UserList>
  <div class="section-end-button">
    <a href="./users">
      <button>
        {$t('home.more-users')}
      </button>
    </a>
  </div>
</LimeSection>

<NightSection arrow>TODO: Clubs</NightSection>

<LimeSection>
  <div class="wrapper">
    <LifetimeStats></LifetimeStats>

    <div class="section-end-button">
      <a href="./coach/{localState.selectedCoach}">
        <button>
          {$t('home.continue-learning-button')}
        </button>
      </a>
    </div>
  </div>
</LimeSection>

<div class="about" style="margin-bottom: {-imageHeight / 2}px">
  <About></About>
</div>

<div class="lowered picture" bind:clientHeight={imageHeight}>
  <img src={asset('/img/jpg/jakob_smile.jpg')} alt="Guy dancing" />
</div>

<Section bgColor={'var(--theme-accent)'} color={'var(--theme-neutral-dark)'}>
  <div class="help-us" style="padding-top: {imageHeight / 2}px">
    <HelpUs />
  </div>
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
  .small-space {
    margin: 3rem 0px 1rem;
  }

  .section-end-button {
    margin: 1rem 0;
  }

  .wrapper {
    min-height: 100dvh;
    /* display: flex; */
    /* flex-direction: column; */
    /* justify-content: center; */
  }

  .picture {
    width: min(90vw, 250px);
    rotate: 7deg;
    margin: auto;
  }
  img {
    width: 100%;
  }

  .lowered {
    margin-top: 5rem;
    transform: translate(15%, 50%);
  }
</style>
