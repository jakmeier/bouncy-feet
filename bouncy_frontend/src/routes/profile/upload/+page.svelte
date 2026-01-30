<script>
  import LogoHeader from '$lib/components/ui/header/LogoHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { fetchMyVideos } from '$lib/peertube';
  import { t, locale, coachLocale } from '$lib/i18n';
  import { page } from '$app/state';
  import { coachData } from '$lib/coach';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import PeertubeVideoPlayer from '$lib/components/ui/video/PeertubeVideoPlayer.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';
  import ScrollToTop from '$lib/components/ScrollToTop.svelte';

  const coachId = $derived(page.url.searchParams.get('coach'));
  const coach = $derived(coachData(coachId || ''));
  const title = $derived(
    $t('profile.upload.title') + ' ' + coach.title[coachLocale($locale)]
  );
</script>

<ScrollToTop />

<LimeSection fillScreen>
  <LogoHeader mainColor {title} />
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    {#snippet children({ apiUser, fullUser, user })}
      <p>
        BouncyFeet public name: {user.publicName}
      </p>
      <p>
        User id: {user.openid}
      </p>

      <VideoUpload {fullUser}></VideoUpload>

      <!-- TODO: pagination, general display etc -->
      <!-- TODO: display private videos -->
      {#await fetchMyVideos()}
        waiting for videos
      {:then videos}
        {#each videos as video}
          <p>
            {video.id} <br />
            {video.createdAt}
            {video.name} <br />
            {video.uuid} <br />
            {#if video.shortUUID}
              <PeertubeVideoPlayer videoId={video.uuid} />
            {/if}
          </p>
        {/each}
      {/await}
    {/snippet}
  </LoginRequiredContent>
</LimeSection>
