<script>
  import { base } from '$app/paths';
  import { t } from '$lib/i18n';
  import FullLogo from '../ui/svg/FullLogo.svelte';
</script>

<div class="logo">
  <FullLogo color="var(--theme-main)"></FullLogo>
</div>

<h1>
  {$t('home.about-title')}
</h1>

<p>
  {$t('home.about-0')}
</p>
<p>
  {$t('home.about-1')}
</p>
<p>
  {$t('home.about-2')}
</p>

<style>
  .logo {
    padding: 2rem;
    max-width: 160px;
    margin: auto;
  }
</style>
