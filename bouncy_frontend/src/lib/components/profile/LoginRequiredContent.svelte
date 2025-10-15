<script>
  import { t } from '$lib/i18n';
  import { getUserContext } from '$lib/context';
  import RequiresLogin from './RequiresLogin.svelte';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { scaleY } from '$lib/sveltex/xtransition';
  import { base } from '$app/paths';
  import { goto } from '$app/navigation';

  let { reason, children } = $props();

  const userContext = getUserContext();
  const user = userContext.store;

  let mounted = $state(false);

  onMount(async () => {
    userContext.clearErrors();
    // TODO: prevent this from firing too often
    if (!userContext.isLoggedInToApi()) {
      try {
        await userContext.pwaAuth.refreshPeerTubeToken();
      } catch (e) {
        console.debug('failed to refresh PeerTube token', e);
      }
    }

    mounted = true;
  });
</script>

{#if userContext.isLoggedInToApi()}
  {@render children?.()}
{:else if mounted}
  <div class="overlay" transition:fade={{ delay: 400, duration: 200 }}>
    <div class="wrapper" transition:scaleY={{ delay: 800 }}>
      {#if userContext.loginError.title !== ''}
        <h2>{$t(userContext.loginError.title)}</h2>
        <div class="error-symbol">
          <img src="{base}/img/symbols/bf_error.svg" alt="bf_error" />
        </div>
        <div class="block">
          {#if userContext.loginError.description}
            <div>{$t(userContext.loginError.description)}</div>
          {/if}
          <div>{$t('common.sorry-error')}</div>
        </div>
        <button onclick={() => history.back()}>
          {$t('common.go-to-prev-page-button')}
        </button>
        <button onclick={() => goto('/')}>
          {$t('common.go-home-button')}
        </button>
        <div class="block">
          <div>{$t('common.report-error')}</div>
        </div>
      {:else}
        <RequiresLogin
          {reason}
          username={$user.publicName}
          openid={$user.openid}
        />
      {/if}
    </div>
  </div>
{:else}
  {$t('profile.waiting-for-login-info')}
{/if}

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
  }

  .wrapper {
    display: flex;
    background-color: var(--background-color, var(--theme-neutral-dark));
    color: var(--color, var(--theme-neutral-gray));
    flex-direction: column;
    min-height: 100px;
    padding: 20px;
    align-items: center;
    text-align: center;
    gap: 10px;
  }

  .error-symbol img {
    max-width: 6rem;
  }

  .block {
    padding: 1rem 0;
  }
</style>
