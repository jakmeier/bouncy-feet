<script>
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import { t } from '$lib/i18n';
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { goto } from '$app/navigation';
  import { ONBOARDING_STATE, register } from '$lib/onboarding';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import { getUserContext } from '$lib/stores/context';

  //   function goToWarmup() {
  //     apiUser.setUserMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_WARMUP);
  //     goto('firstCourse');
  //   }

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let showTitle = $state(false);
  let showText = $state(false);
  let showButtons = $state(false);

  let showQuestion = $state(true);
  let showFirstVisit = $state(false);
  let showReturningVisitor = $state(false);
  let showLogin = $state(false);

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
      showButtons = true;
    }, 2900);
  }

  function toFirstVisit() {
    showQuestion = false;
    showText = false;
    showButtons = false;

    setTimeout(() => {
      showFirstVisit = true;
    }, 500);
    setTimeout(() => {
      showTitle = true;
    }, 1000);
    setTimeout(() => {
      showText = true;
    }, 2000);
  }

  function toReturningVisitor() {
    showQuestion = false;
    showText = false;
    showButtons = false;

    setTimeout(() => {
      showReturningVisitor = true;
    }, 1000);
  }

  function showButtonsDelayed() {
    setTimeout(() => {
      showButtons = true;
    }, 500);
  }

  function toLogin() {
    showReturningVisitor = false;

    setTimeout(() => {
      showLogin = true;
    }, 1000);
  }
</script>

<div class="wrapper">
  {#if showQuestion}
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
          {$t('home.first-visit-question')}
        </h1>
        {#if showButtons}
          <div class="buttons">
            <button
              class="full-width"
              in:slide|fade
              out:fade
              onclick={toFirstVisit}
            >
              {$t('home.first-visit-button')}
            </button>
            <button
              class="full-width"
              in:slide|fade
              out:fade
              onclick={toReturningVisitor}
            >
              {$t('home.not-first-visit-button')}
            </button>
          </div>
        {/if}
      </div>
    {/if}
  {/if}

  {#if showReturningVisitor}
    <NightSection>
      <div transition:slide={{ duration: 1000 }}>
        <LogoHeader />
      </div>

      <div class="text">
        <h1 in:slide|fade out:fade>
          {$t('home.not-first-visit-title')}
        </h1>

        <p>
          {$t('home.not-first-visit-text-1')}
        </p>

        <button class="full-width" in:slide|fade out:fade onclick={toLogin}>
          {$t('profile.button-login')}
        </button>

        <h1 in:slide|fade out:fade>
          {$t('home.no-login-title')}
        </h1>

        <p>
          {$t('home.not-first-visit-text-4')}
        </p>

        <div class="buttons">
          <button class="full-width" onclick={userCtx.createGuestUser}>
            {$t('profile.button-create-guest')}
          </button>

          <button class="full-width" onclick={register}>
            {$t('profile.button-register')}
          </button>
        </div>

        <p>
          {$t('home.not-first-visit-text-2')}
        </p>
        <p>
          {$t('home.not-first-visit-text-3')}
        </p>
      </div>

      <div class="footer" transition:slide={{ duration: 1000 }}>
        <Footer white />
      </div>
    </NightSection>
  {/if}

  {#if showFirstVisit}
    <NightSection>
      <div transition:slide={{ duration: 500 }}>
        <LogoHeader />
      </div>

      <div class="text">
        {#if showTitle}
          <h1 in:slide|fade out:fade>Welcome</h1>
        {/if}
        {#if showText}
          <p in:fade out:fade>{$t('home.first-visit-welcome')}</p>
          <p in:fade out:fade onintroend={showButtonsDelayed}>
            {$t('home.first-visit-description')}
          </p>
        {/if}
        {#if showButtons}
          <div class="buttons" transition:fade>
            <button class="full-width" onclick={userCtx.createGuestUser}>
              {$t('profile.button-create-guest')}
            </button>

            <button class="full-width" onclick={register}>
              {$t('profile.button-register')}
            </button>
          </div>
        {/if}
      </div>
      <div class="fixed-footer" transition:slide={{ duration: 500 }}>
        <Footer white />
      </div>
    </NightSection>
  {/if}

  {#if showLogin}
    <LoginRequiredContent>
      {#snippet guest({ apiUser })}
        TODO
      {/snippet}
    </LoginRequiredContent>
  {/if}
</div>

<style>
  @keyframes intro-animation {
    0% {
      visibility: hidden;
      height: 0%;
    }
    100% {
      height: 100%;
      visibility: visible;
    }
  }
  .wrapper {
    position: relative;
    height: 100dvh;
    margin: 0;
    color: var(--theme-main);
  }
  .text {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    /* animation: intro-animation 500ms; */
  }

  .fixed-footer {
    position: absolute;
    bottom: 1rem;
    z-index: -1;
  }
</style>
