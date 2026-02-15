<script>
  import { page } from '$app/state';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import { t } from '$lib/i18n';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const userId = Number.parseInt(page.params.userId || '0');
</script>

<LimeSection>
  <BackHeader title={data.displayedUser.display_name} mainColor></BackHeader>
  <div class="pic">
    <ActorAvatar actor={data.displayedUser.account || undefined} />
  </div>
  <div class="description">{data.displayedUser.account?.description}</div>
</LimeSection>

<NightSection>
  <h1>{$t('profile.combos-title')}</h1>
  TODO: User videos / combos etc
  <!-- {#if clubDetails.private}
    <h2>{$t('club.private-videos-title')}</h2>
    {#each clubDetails.private.private_playlists as playlist, i}
      <div class="playlist">
        <Playlist
          bind:this={privatePlaylists[i]}
          playlistInfo={playlist}
          editable={isClubMember}
        />
      </div>
    {/each} 
  {/if}-->
  <Footer white />
</NightSection>

<style>
  .pic {
    display: grid;
    justify-content: center;
  }

  .description {
    margin: 1rem 0;
  }
</style>
