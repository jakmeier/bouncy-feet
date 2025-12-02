<script>
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { fetchMyVideos } from '$lib/peertube';
  import { t, locale, coachLocale } from '$lib/i18n';
  import { page } from '$app/state';
  import { coachData } from '$lib/coach';
  import { getUserContext } from '$lib/context';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import PeertubeVideoPlayer from '$lib/components/ui/video/PeertubeVideoPlayer.svelte';
  import VideoUpload from '$lib/components/ui/video/VideoUpload.svelte';

  /** @type {UserContextData} */
  const userCtx = getUserContext();
  const user = userCtx.store;
  const pwaAuth = userCtx.pwaAuth;

  const coachId = $derived(page.url.searchParams.get('coach'));
  const coach = $derived(coachData(coachId || ''));
  const title = $derived(
    $t('profile.upload.title') + ' ' + coach.title[coachLocale($locale)]
  );
</script>

<LimeSection fillScreen>
  <LogoHeader mainColor {title} />
  <LoginRequiredContent
    reason={$t('profile.upload.requires-login-description')}
  >
    <p>
      BouncyFeet public name: {$user.publicName}
    </p>
    <p>
      User id: {$user.openid}
    </p>

    <VideoUpload></VideoUpload>

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
  </LoginRequiredContent>
</LimeSection>
