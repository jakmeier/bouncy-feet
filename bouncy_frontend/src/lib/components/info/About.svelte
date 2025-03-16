<script>
  import { base } from '$app/paths';
  import { t } from '$lib/i18n';
  import FullLogo from '../ui/FullLogo.svelte';
</script>

<div class="logo">
  <FullLogo color="var(--theme-main)"></FullLogo>
</div>

<p>
  {$t('home.about-0')}
</p>
<p>
  {$t('home.about-1')}
</p>
<p>
  {$t('home.about-2')}
</p>

<div class="lowered picture">
  <img src="{base}/img/jpg/jakob_smile.jpg" alt="Guy dancing" />
</div>

<style>
  .logo {
    padding: 2rem;
    max-width: 160px;
    margin: auto;
  }

  .picture {
    width: min(50vw, 200px);
    rotate: 7deg;
    margin: auto;
  }
  img {
    width: 100%;
  }

  .lowered {
    transform: translateY(5rem);
    margin-top: -4rem;
  }
</style>
