<script>
  import { page } from '$app/state';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import EmptyUserText from '$lib/components/user/EmptyUserText.svelte';
  import UserCombos from '$lib/components/user/UserCombos.svelte';
  import { t } from '$lib/i18n';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const userId = Number.parseInt(page.params.userId || '0');

  let hasCombos = $state();
  let isEmpty = $derived(
    hasCombos === false && !data.displayedUser.account?.description
  );
  let name = $derived(
    data.displayedUser.display_name ||
      data.displayedUser.peertube_handle ||
      $t('profile.title')
  );
</script>

<LimeSection>
  <BackHeader title={name} mainColor></BackHeader>
  <div class="pic">
    <ActorAvatar
      actor={data.displayedUser.account || undefined}
      seed={userId}
    />
  </div>
  <div class="description">{data.displayedUser.account?.description}</div>
</LimeSection>

<NightSection>
  {#if isEmpty}
    <p>
      <EmptyUserText {name}></EmptyUserText>
    </p>
  {/if}

  {#if hasCombos}
    <h1>{$t('profile.combos-title')}</h1>
  {/if}
  <UserCombos {userId} bind:hasContent={hasCombos}></UserCombos>
  <Footer white />
</NightSection>

<style>
  .pic {
    display: grid;
    justify-content: center;
    min-width: 192px;
    min-height: 192px;
  }

  .description {
    margin: 1rem 0;
  }
</style>
