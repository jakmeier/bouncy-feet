<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import { base } from '$app/paths';
  import { features } from '$lib/stores/FeatureSelection';

  export let height = 100;

  $: tabs = buildTabs($features);

  /** @param {Features} features */
  function buildTabs(features) {
    const tabs = [];
    tabs.push({ label: $t('home.nav'), icon: 'home', route: `${base}/` });
    if (features.enableFreestyleRecording) {
      tabs.push({
        label: $t('record.nav'),
        icon: 'directions_walk',
        route: `${base}/record`,
      });
    }
    tabs.push({
      label: $t('collection.nav'),
      icon: 'book_5',
      route: `${base}/collection`,
    });
    tabs.push({
      label: $t('profile.nav'),
      icon: 'account_circle',
      route: `${base}/profile`,
    });
    if (features.enableDevView) {
      tabs.push({ label: 'Dev', icon: 'code', route: `${base}/dev` });
    }
    return tabs;
  }
</script>

<nav class="navbar" style="height:{height}px">
  {#each tabs as { label, route, icon }}
    <a
      class="tab"
      class:active-tab={$page.url.pathname === route}
      href={route}
      title={label}
    >
      <span class="material-symbols-outlined">{icon}</span>
    </a>
  {/each}
</nav>

<style>
  .navbar {
    display: flex;
    flex-flow: row;
    justify-content: space-around;
    overflow: hidden;
    background-color: var(--theme-neutral-dark);
    position: fixed;
    bottom: 0;
    width: 100%;
  }

  a.tab {
    display: grid;
    grid-template-columns: auto;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.3s ease;
    border-radius: 5px;
    color: var(--theme-neutral-light);
    text-decoration: none;
  }

  .tab span {
    margin: auto;
    font-size: 50px;
  }

  .tab:hover {
    color: var(--theme-accent);
  }

  a.active-tab {
    color: var(--theme-neutral-white);
  }
</style>
