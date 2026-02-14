<script>
  import { page } from '$app/state';
  import ActorAvatar from '$lib/components/profile/ActorAvatar.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import BackHeader from '$lib/components/ui/header/BackHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';

  /** @type {import('./$types').PageProps} */
  let { data } = $props();

  const userId = Number.parseInt(page.params.userId || '0');
</script>

<LimeSection>
  <BackHeader title={data.displayedUser.display_name} mainColor></BackHeader>
  <ActorAvatar actor={data.displayedUser.account || undefined} />
</LimeSection>

<NightSection>
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
