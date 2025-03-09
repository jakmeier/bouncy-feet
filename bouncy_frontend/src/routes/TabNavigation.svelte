<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import { base } from '$app/paths';
  import { features } from '$lib/stores/FeatureSelection';

  /**
   * @typedef {Object} Props
   * @property {number} [height]
   */

  /** @type {Props} */
  let { height = 100 } = $props();


  /** @param {Features} features */
  function buildTabs(features) {
    const tabs = [];
    tabs.push({ label: $t('home.nav'), icon: 'home', route: `${base}/` });
    if (features.enableCourses) {
      tabs.push({
        label: $t('courses.nav'),
        icon: 'school',
        route: `${base}/courses`,
      });
    }
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
    if (features.enableEditorPage) {
      tabs.push({
        label: $t('editor.nav'),
        icon: 'experiment',
        route: `${base}/editor`,
      });
    }
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
  let tabs = $derived(buildTabs($features));
</script>

<div class="nav-background">
  <nav class="navbar" style="height:{height}px">
    {#each tabs as { label, route, icon }}
      <a
        class="tab"
        class:active-tab={page.url.pathname === route}
        href={route}
        title={label}
      >
        <span class="material-symbols-outlined" translate="no">{icon}</span>
      </a>
    {/each}
  </nav>
</div>

<style>
  .nav-background {
    overflow: hidden;
    background-color: var(--theme-main);
    position: fixed;
    bottom: 0;
    width: 100%;
    z-index: 101;
  }

  .navbar {
    max-width: 720px;
    margin: auto;
    display: flex;
    flex-flow: row;
    justify-content: space-around;
  }

  a.tab {
    display: grid;
    grid-template-columns: auto;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.3s ease;
    border-radius: 5px;
    color: var(--theme-neutral-dark);
    text-decoration: none;
  }

  .tab span {
    margin: auto;
    font-size: 50px;
  }

  .tab:hover {
    color: var(--theme-main-alt);
  }

  a.active-tab {
    color: var(--theme-neutral-white);
  }
</style>
