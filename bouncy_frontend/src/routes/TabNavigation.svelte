<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';

  const tabs = [
    { label: $t('home.nav'), icon: 'home', route: '/' },
    { label: $t('learn.nav'), icon: 'book_5', route: '/learn' },
    { label: $t('record.nav'), icon: 'directions_walk', route: '/record' },
    { label: $t('profile.nav'), icon: 'account_circle', route: '/profile' },
  ];
</script>

<nav class="navbar">
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
    display: grid;
    grid-template-columns: auto auto auto auto;
    overflow: hidden;
    background-color: var(--theme-neutral-dark);
    position: fixed;
    bottom: 0;
    width: 100%;
    height: 100px;
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
    color: var(--theme-accent-light);
  }
</style>
