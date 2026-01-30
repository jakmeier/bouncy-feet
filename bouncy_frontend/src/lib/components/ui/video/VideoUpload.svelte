<script>
  import { uploadVideoToPeerTubeResumable, VIDEO_PRIVACY } from '$lib/peertube';
  import { t } from '$lib/i18n';
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';

  /**
   * @typedef {Object} Props
   * @prop {(video: import('$lib/peertube-openapi').VideoUploadResponse)=>void} [onVideoUploaded]
   * @prop {import('$lib/peertube-openapi').VideoPrivacySet} [privacy]
   * @prop {FullUser} fullUser
   */

  /** @type {Props}*/
  let {
    onVideoUploaded = () => {},
    privacy = VIDEO_PRIVACY.PRIVATE,
    fullUser,
  } = $props();

  let file = $state(null);
  let isUploading = $state(false);
  let uploadProgress = $state(0);
  let error = $state('');
  let fileInput = $state();

  export function open() {
    fileInput.click();
  }

  /**
   * @returns {Promise<number>}
   */
  async function getUserChannel() {
    // TODO
    const ptu = await fullUser.peerTubeUser;
    const channels = ptu?.videoChannels;
    // @ts-ignore
    const channelId = channels[0]?.id;
    if (channelId === undefined) {
      // TODO: create it!
      throw new Error('No video channel found');
    }
    return channelId;
  }

  async function handleFileSelect(event) {
    file = event.target.files?.[0] ?? null;
    error = '';

    if (!file) {
      console.info('no file selected');
      return;
    }

    // TODO
    // let accessToken = pwaAuth.peerTubeToken?.access_token;
    // if (!accessToken) {
    //   await pwaAuth.refreshPeerTubeToken();
    //   accessToken = pwaAuth.peerTubeToken?.access_token;
    // }

    // if (!accessToken) {
    //   console.error('No PeerTube access token');
    //   // TODO: translate error to user
    //   error =
    //     'Uhm, failed authentication with video hosting service, sorry about that :(';
    //   return;
    // }

    isUploading = true;
    uploadProgress = 0;

    try {
      let channelId = await getUserChannel();

      // upload in background
      uploadVideoToPeerTubeResumable(
        file,
        channelId,
        (ratio) => (uploadProgress = Math.floor(ratio * 100)),
        privacy
      )
        .then((videoResponse) => {
          uploadProgress = 100;
          onVideoUploaded(videoResponse);
        })
        .catch((err) => {
          console.error(err);
          // TODO: translate error to user
          error = "Oops, I couldn't upload that :(";
        })
        .finally(() => {
          isUploading = false;
        });
    } catch (err) {
      console.error(err);
      // TODO: translate error to user
      error = "Oops, I couldn't upload that :(";
      isUploading = false;
    }
  }
</script>

<!-- TODO: Is a nested Login-required a good idea? -->
<LoginRequiredContent reason={$t('profile.upload.requires-login-description')}>
  <input
    bind:this={fileInput}
    type="file"
    accept="video/*"
    onchange={handleFileSelect}
  />

  {#if isUploading}
    <p>Uploading… {uploadProgress}%</p>
  {/if}

  {#if error}
    <p style="color: red">{error}</p>
  {/if}
</LoginRequiredContent>

<style>
  input[type='file'] {
    margin: 1rem 0;
  }
</style>
