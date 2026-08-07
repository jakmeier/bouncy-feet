<script>
  import '../app.css';
  import { t } from '$lib/i18n.js';

  /**
   * @typedef {Object} Props
   * @property {import('svelte').Snippet} [children]
   * @property {any} data
   */

  /** @type {Props} */
  let { children, data } = $props();
</script>

<svelte:head>
  <title>{$t('meta.title')}</title>
</svelte:head>

{#if data.isQrShell}
  <div class="qr-background">
    <main>
      {@render children?.()}
    </main>
  </div>
{:else}
  {#await import('./AppShell.svelte') then module}
    {@const AppShell = module.default}
    <AppShell {data}>
      {@render children?.()}
    </AppShell>
  {/await}
{/if}

<style>
  .qr-background {
    min-height: 100vh;
    background-color: var(--background-color);
    color: var(--color);
  }

  .qr-background main {
    min-height: 100vh;
  }
</style>
