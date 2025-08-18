<script>
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import LimeSection from '$lib/components/ui/sections/LimeSection.svelte';
  import { loginToPeertube, uploadVideoToPeerTube } from '$lib/peertube';
  import { pwaAuth } from '$lib/stores/Auth.svelte';
  import { getContext } from 'svelte';
  import { t, locale, coachLocale } from '$lib/i18n';
  import RequiresLoginPopup from '$lib/components/profile/RequiresLoginPopup.svelte';
  import { page } from '$app/state';
  import { coachData } from '$lib/coach';

  /** @type {UserContextData} */
  const { store: user, setUserMeta } = getContext('user');

  const coachId = $derived(page.url.searchParams.get('coach'));
  const coach = $derived(coachData(coachId || ''));
  const title = $derived(
    $t('profile.upload.title') + ' ' + coach.title[coachLocale($locale)]
  );

  let file = $state(null);
  let isUploading = $state(false);
  let uploadProgress = $state(0);
  let error = $state('');

  async function handleFileSelect(event) {
    file = event.target.files?.[0] ?? null;
    error = '';

    if (!file) {
      console.info('no file selected');
      return;
    }

    let accessToken = pwaAuth.peerTubeToken?.access_token;
    if (!accessToken) {
      await loginToPeertube();
      accessToken = pwaAuth.peerTubeToken?.access_token;
    }

    if (!accessToken) {
      console.error('No PeerTube access token');
      // TODO: translate error to user
      error =
        'Uhm, failed authentication with video hosting service, sorry about that :(';
      return;
    }

    isUploading = true;
    uploadProgress = 0;

    try {
      // TODO: Can I track uploading progress somehow?
      await uploadVideoToPeerTube(file, accessToken);
      uploadProgress = 100;
    } catch (err) {
      console.error(err);
      // TODO: translate error to user
      error = "Oops, I couldn't upload that :(";
    } finally {
      isUploading = false;
    }
  }
</script>

<RequiresLoginPopup reason={$t('profile.upload.requires-login-description')} />

<LimeSection fillScreen>
  <LogoHeader mainColor {title} />

  <!-- WIP / TODO: These two should be the same (July) -->
  <p>
    BouncyFeet public name: {$user.publicName}
  </p>
  <p>
    PeerTube user name: {pwaAuth.userProfile?.username}
  </p>

  <input type="file" accept="video/*" onchange={handleFileSelect} />

  {#if isUploading}
    <p>Uploading… {uploadProgress}%</p>
  {/if}

  {#if error}
    <p style="color: red">{error}</p>
  {/if}
</LimeSection>

<style>
  input[type='file'] {
    margin: 1rem 0;
  }
</style>
