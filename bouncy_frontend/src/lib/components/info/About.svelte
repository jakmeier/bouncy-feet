<script>
  import { base } from '$app/paths';
  import { t } from '$lib/i18n';
  import FullLogo from '../ui/FullLogo.svelte';
</script>

<div class="logo">
  <FullLogo color="var(--theme-main)"></FullLogo>
</div>

<p>
  {$t('home.about-title')}
</p>

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
