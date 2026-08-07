<script>
  import { PUBLIC_APP_URL } from '$env/static/public';
  import { t } from '$lib/i18n.js';
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';

  let { data } = $props();
  let visible = $state(false);

  // Intro transitions only run when their elements enter the DOM. Render this
  // shell on the server, then insert its content after client hydration.
  onMount(() => {
    visible = true;
  });
</script>

<svelte:head>
  <title>{$t('qr.meta.title')}</title>
  <meta name="description" content={$t('qr.meta.description')} />
</svelte:head>

<section class="qr-landing" aria-labelledby="qr-title">
  {#if visible}
    <h1 id="qr-title" class="title" in:fade={{ duration: 1000 }}>
      {$t('qr.title')}
    </h1>
    <a
      class="app-link"
      href={PUBLIC_APP_URL}
      in:fade={{ duration: 450, delay: 1000 }}
    >
      {$t('qr.open-app')}
    </a>
    <p class="reference" in:fade={{ duration: 450, delay: 1500 }}>
      <span>{$t('qr.reference')}</span>
      {data.code}
    </p>
  {/if}
</section>

<style>
  .qr-landing {
    box-sizing: border-box;
    display: grid;
    place-items: center;
    align-content: center;
    min-height: 100vh;
    padding: 1.5rem;
    text-align: center;
  }

  h1 {
    margin: 0 0 2rem;
  }

  .title {
    white-space: pre-line;
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

  .reference {
    max-width: min(100%, 48rem);
    margin-top: 2rem;
    color: var(--theme-neutral-white-60);
    font-size: var(--font-small);
    line-height: 1.5;
    overflow-wrap: anywhere;
    word-break: break-word;
  }

  .reference span {
    white-space: nowrap;
  }
</style>
