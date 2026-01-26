<script>
  import { run } from 'svelte/legacy';

  import DanceStats from './DanceStats.svelte';
  import { t } from '$lib/i18n';
  import Toggle from '$lib/components/ui/Toggle.svelte';
  import { dev, displayedVersion } from '$lib/stores/FeatureSelection';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import { getUserContext } from '$lib/context';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';
  import { fetchMyVideos } from '$lib/peertube';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import ThumbnailJuggler from '$lib/components/ui/ThumbnailJuggler.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';
  import Clubs from '$lib/components/club/Clubs.svelte';
  import CreateClub from '$lib/components/club/CreateClub.svelte';

  /** @type {UserContextData} */
  const { store: user, setUserMeta, logout } = getUserContext();
  // let showStatsSharingPopup = $state(writable(!$user.consentSendingStats));

  async function submit() {
    setUserMeta('publicName', $user.publicName);
  }

  // function consent(yes) {
  //   if (yes === true) {
  //     $user.consentSendingStats = true;
  //   }
  //   $showStatsSharingPopup = false;
  // }

  // function openSettings() {
  //   goto('./settings');
  // }

  let unlockHiddenFeatures = $state(0);
  let lastIncrease = performance.now();
  function clickProfile() {
    if (lastIncrease + 500 > performance.now()) {
      unlockHiddenFeatures += 1;
    } else {
      unlockHiddenFeatures = 0;
    }
    lastIncrease = performance.now();
  }
  let devFeaturesOn = $state($dev);
  let version005 = $state(false);
  let standardVersion = $displayedVersion;
  run(() => {
    devFeaturesOn ? $dev || window.toggleDev() : $dev && window.toggleDev();
  });
  run(() => {
    version005, displayedVersion.set(version005 ? 0.005 : standardVersion);
  });
</script>

<ScrollToTop />

<DarkSection fillScreen arrow>
  <LogoHeader
    title={$t('profile.title')}
    backButton={false}
    homeLink
    onAction={logout}
    button="logout"
  />

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="profile-pic" onclick={clickProfile}>
    <Symbol size={100}>person</Symbol>
    {$user.publicName}
  </div>
  <h2 class="box">{$t('profile.stats-title')}</h2>
  <DanceStats
    seconds={$user.recordedSeconds}
    numSteps={$user.recordedSteps}
    numDances={$user.recordedDances}
  />

  <form class="inputs">
    <label for="publicName">{$t('profile.public-name')}</label>
    <input id="publicName" type="text" bind:value={$user.publicName} />
    <button onclick={submit} class="wide"
      >{$t('profile.update-name-button')}</button
    >
  </form>
</DarkSection>

<LimeSection arrow fillScreen>
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    <h2>{$t('profile.my-videos-title')}</h2>
    {#await fetchMyVideos()}
      waiting for videos
    {:then videos}
      <div class="videos">
        {#if videos.length === 0}
          <p>{$t('video.empty-playlist')}</p>
        {:else}
          <ThumbnailJuggler {videos} userExtraInfo />
        {/if}
      </div>
    {/await}
  </LoginRequiredContent>
</LimeSection>

<LightSection arrow>
  <h2>{$t('club.upload-video-button')}</h2>
  <p>{$t('profile.upload-video-description')}</p>
  <VideoUpload></VideoUpload>
</LightSection>

<NightSection>
  <div class="private">
    <h2>{$t('club.my-clubs-title')}</h2>
    <p>{$t('club.description-0')}</p>
    <p>{$t('club.description-1')}</p>
    <p>{$t('club.description-2')}</p>
    <Clubs />
  </div>

  <h2>{$t('club.create-new-title')}</h2>
  <CreateClub></CreateClub>
  <Footer white />
</NightSection>

<!-- <Popup title={'profile.consent.title'} bind:isOpen={showStatsSharingPopup}>
  <div>{$t('profile.consent.text0')}</div>
  <div>{$t('profile.consent.question')}</div>

  <div class="buttons">
    <button onclick={() => consent(true)}>
      <p>{$t('profile.consent.yes')}</p>
    </button>

    <button onclick={() => consent(false)}>
      <p>{$t('profile.consent.no')}</p>
    </button>
  </div>
</Popup> -->

{#if unlockHiddenFeatures > 6}
  <div class="hidden-features">
    <div>Dev Features</div>
    <Toggle bind:isOn={devFeaturesOn} />
    <div>Version 0.005</div>
    <Toggle bind:isOn={version005} />
  </div>
{/if}

<style>
  .profile-pic {
    display: grid;
    grid-template-columns: max-content;
    justify-content: center;
    font-weight: 900;
    text-align: center;
  }

  .inputs {
    /* display grid allows to fit input field to width */
    display: grid;
    gap: 1rem;
    margin: 2rem 0;
  }

  .buttons {
    display: grid;
    grid-template-columns: auto auto;
    gap: 2rem;
  }

  .hidden-features {
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    border-radius: 10px;
    padding: 10px;
    text-align: center;
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    gap: 10px;
    margin: 5px;
  }

  .private {
    margin-bottom: 2rem;
  }

  .videos {
    margin: auto;
    height: 240px;
    width: min(280px, calc(100vw - 3rem)); /* PeerTube thumbnail width */
  }
</style>
