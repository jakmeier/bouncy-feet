<script>
  import Background from '$lib/components/ui/sections/Background.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import { t } from '$lib/i18n';

  let { showResults = $bindable() } = $props();
</script>

<Background
  bgColor="var(--theme-neutral-dark)"
  color="var(--theme-neutral-white)"
></Background>

<StandardPage gray>
  <!-- <h3>That was fun, wasn't it?</h3> -->
  <div class="outer">
    <DarkSection>
      <div class="inner">
        <Symbol size={155}>trophy</Symbol>

        <button onclick={() => (showResults = true)}
          >{$t('record.results-button')}</button
        >
      </div>
    </DarkSection>
  </div>
</StandardPage>

<style>
  .inner {
    position: relative;
    height: 50dvh;
    display: grid;
    justify-content: center;
    justify-items: center;
    align-items: center;
  }
  .outer {
    position: relative;
    margin: 4rem 0;
  }
  /* h3 {
    margin: 2rem 0;
  } */
</style>
