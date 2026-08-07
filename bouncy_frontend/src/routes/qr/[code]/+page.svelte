<script>
  import { PUBLIC_APP_URL } from '$env/static/public';
  import { t } from '$lib/i18n.js';
  import { resolveQrTarget } from '$lib/qrResolver.js';
  import { onMount } from 'svelte';

  let { data } = $props();
  let resolutionState = $state('opening');

  // onMount runs after the server-rendered forwarding view is visible. Hashing
  // and the digest-only lookup therefore never run during SSR or before the
  // recovery link has been rendered.
  onMount(() => {
    const controller = new AbortController();
    let isCurrent = true;

    void resolveQrTarget(data.code, { signal: controller.signal })
      .then((targetUrl) => {
        if (isCurrent) {
          window.location.replace(targetUrl);
        }
      })
      .catch(() => {
        if (isCurrent) {
          resolutionState = 'fallback';
        }
      });

    return () => {
      isCurrent = false;
      controller.abort();
    };
  });
</script>

<svelte:head>
  <title>{$t('qr.meta.title')}</title>
  <meta name="description" content={$t('qr.meta.description')} />
</svelte:head>

<section class="qr-landing" aria-labelledby="qr-title">
  <div class="qr-card">
    <p class="brand">Bouncy Feet</p>
    {#if resolutionState === 'opening'}
      <div class="spinner" aria-hidden="true"></div>
    {/if}
    <h1 id="qr-title" class="title">
      {$t('qr.title')}
    </h1>
    <p class="status" aria-live="polite">
      {resolutionState === 'opening' ? $t('qr.opening') : $t('qr.unavailable')}
    </p>
    {#if resolutionState === 'fallback'}
      <p class="reference">
        <span>{$t('qr.reference')}</span>
        {data.code}
      </p>
    {/if}
    <a class="app-link" href={PUBLIC_APP_URL}>{$t('qr.open-app')}</a>
  </div>
</section>

<style>
  .qr-landing {
    box-sizing: border-box;
    display: grid;
    min-height: 100vh;
    place-items: center;
    padding: 1.5rem;
  }

  .qr-card {
    box-sizing: border-box;
    width: min(100%, 28rem);
    padding: 2.5rem 2rem;
    border-radius: 1.5rem;
    background: var(--theme-neutral-almost-black);
    text-align: center;
  }

  .brand {
    margin: 0 0 2rem;
    color: var(--theme-main);
    font-size: var(--font-normal);
    font-weight: 700;
  }

  h1 {
    margin: 1.5rem 0 0;
  }

  .title {
    white-space: pre-line;
  }

  .spinner {
    box-sizing: border-box;
    width: 3rem;
    height: 3rem;
    margin: 0 auto;
    border: 0.3rem solid var(--theme-neutral-white-60);
    border-top-color: var(--theme-main);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .status {
    margin: 0.75rem 0 2rem;
    color: var(--theme-neutral-white-60);
  }

  .reference {
    margin: -1rem 0 2rem;
    color: var(--theme-neutral-white-60);
    font-size: var(--font-small);
    line-height: 1.5;
    overflow-wrap: anywhere;
    word-break: break-word;
  }

  .reference span {
    white-space: nowrap;
  }

  .app-link {
    box-sizing: border-box;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 44px;
    min-height: 44px;
    padding: 0.75rem 1rem;
    border-radius: 999px;
    background: var(--theme-accent);
    color: var(--theme-neutral-black);
    font-size: var(--font-normal);
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .spinner {
      animation: none;
    }
  }
</style>
