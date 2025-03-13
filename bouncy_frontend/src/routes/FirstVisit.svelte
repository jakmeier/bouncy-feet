<script>
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import { t } from '$lib/i18n';
  import { getContext, onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { goto } from '$app/navigation';
  import { ONBOARDING_STATE } from '$lib/onboarding';

  /** @type {UserContextData}*/
  const { setUserMeta } = getContext('user');

  function goToWarmup() {
    setUserMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_WARMUP);
    goto('firstWarmup');
  }

  let showTitle = false;
  let showText = false;
  let showButton = false;
  let showLogo = false;
  let showVideo = false;

  /** @type {Video} */
  let video;

  onMount(() => {
    // triggers first animation
    showTitle = true;
  });

  function moreText() {
    setTimeout(() => {
      showTitle = false;
    }, 1000);

    setTimeout(() => {
      showText = true;
    }, 2000);

    setTimeout(() => {
      showButton = true;
    }, 4000);
  }

  function toVideo() {
    showText = false;
    showButton = false;

    setTimeout(() => {
      showLogo = true;
    }, 1000);

    setTimeout(() => {
      showVideo = true;
    }, 2000);
  }
</script>

<div class="wrapper">
  {#if showLogo}
    <div transition:slide={{ duration: 1000 }}>
      <LogoHeader />
    </div>
  {/if}

  {#if showTitle}
    <div class="text">
      <h1 in:slide|fade out:fade onintroend={moreText}>
        {$t('home.first-visit-0')}
      </h1>
    </div>
  {/if}
  {#if showText}
    <div class="text">
      <h1 in:slide|fade out:fade>
        {$t('home.first-visit-1')}
      </h1>
      {#if showButton}
        <button in:slide|fade out:fade onclick={toVideo}>
          {$t('home.first-visit-button-0')}
        </button>
      {/if}
    </div>
  {/if}
  {#if showVideo}
    <div
      class="centered"
      in:slide={{ duration: 500, axis: 'x' }}
      out:fade
      onintroend={() => {
        video.startVideo();
      }}
    >
      <Video
        controls={false}
        path="https://app.bouncy-feet.ch/media/videos/wip/l1.mp4"
        bind:this={video}
      ></Video>
      <button onclick={goToWarmup}>
        {$t('home.first-visit-button-1')}
      </button>
    </div>
  {/if}

  {#if showLogo}
    <div class="footer" transition:slide={{ duration: 1000 }}>
      <Footer white />
    </div>
  {/if}
</div>

<style>
  .wrapper {
    position: relative;
    height: 100dvh;
    margin: 0;
  }
  .text {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .centered {
    text-align: center;
  }

  .centered button {
    margin: 1rem;
  }

  .footer {
    position: absolute;
    bottom: 1rem;
    z-index: -1;
  }
</style>
