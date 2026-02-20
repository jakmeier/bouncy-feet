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

  //   function goToWarmup() {
  //     apiUser.setUserMeta('onboarding', ONBOARDING_STATE.STARTED_FIRST_WARMUP);
  //     goto('firstCourse');
  //   }

  /**
   * @typedef {Object} Props
   * @property {()=>void} createGuest
   */

  /** @type {Props} */
  let { createGuest } = $props();

  let showTitle = $state(false);
  let showQuestion = $state(false);
  let showQuestionButtons = $state(false);
  let showLogo = $state(false);
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
      showQuestion = true;
    }, 2000);

    setTimeout(() => {
      showQuestionButtons = true;
    }, 2900);
  }

  function toFirstVisit() {
    showQuestion = false;
    showQuestionButtons = false;

    setTimeout(() => {
      showLogo = true;
    }, 1000);

    setTimeout(() => {
      showFirstVisit = true;
    }, 2000);
  }

  function toReturningVisitor() {
    showQuestion = false;
    showQuestionButtons = false;

    setTimeout(() => {
      showLogo = true;
    }, 1000);

    setTimeout(() => {
      showReturningVisitor = true;
    }, 2000);
  }

  function toLogin() {
    showReturningVisitor = false;

    setTimeout(() => {
      showLogin = true;
    }, 1000);
  }
</script>

<div class="wrapper">
  {#if showTitle}
    <div class="text">
      <h1 in:slide|fade out:fade onintroend={moreText}>
        {$t('home.first-visit-0')}
      </h1>
    </div>
  {/if}
  {#if showQuestion}
    <div class="text">
      <h1 in:slide|fade out:fade>
        {$t('home.first-visit-question')}
      </h1>
      {#if showQuestionButtons}
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
          <button class="full-width" onclick={createGuest}>
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
    TODO: you may create a login or continue as guest and create a login later.
    You can only access the guest account on this device and it can't be
    recovered if you lost access to this device.
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
  }

  .footer {
    /* position: absolute; */
    /* bottom: 1rem; */
    /* z-index: -1; */
  }
</style>
