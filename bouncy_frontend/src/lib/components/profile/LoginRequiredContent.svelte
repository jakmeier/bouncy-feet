<script>
  import { t } from '$lib/i18n';
  import { getUserContext } from '$lib/context';
  import RequiresLogin from './RequiresLogin.svelte';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import { scaleY } from '$lib/sveltex/xtransition';

  let { reason, children } = $props();

  const userContext = getUserContext();
  const user = userContext.store;

  let mounted = $state(false);

  onMount(async () => {
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
      <RequiresLogin
        {reason}
        username={$user.publicName}
        openid={$user.openid}
      />
    </div>
  </div>
{:else}
  {$t('waiting-for-login-info')}
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
</style>
