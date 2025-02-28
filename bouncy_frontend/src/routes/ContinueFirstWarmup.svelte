<script>
  import { goto } from '$app/navigation';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import { t } from '$lib/i18n';
</script>

<div class="wrapper">
  <LogoHeader />
  <h3>{$t('home.continue-first-warmup')}</h3>
  <button onclick={() => goto('firstWarmup')}>
    {$t('home.continue-button')}
  </button>

  <div class="footer">
    <Footer white />
  </div>
</div>

<style>
  .wrapper {
    position: relative;
    height: 100dvh;
    margin: 0;
  }

  .footer {
    position: absolute;
    bottom: 1rem;
  }
</style>
