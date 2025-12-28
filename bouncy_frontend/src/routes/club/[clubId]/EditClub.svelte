<script>
  import ActorAvatar from './ActorAvatar.svelte';
  import * as api from '$lib/peertube-openapi';
  import ImageCropper from '$lib/components/ui/ImageCropper.svelte';
  import { getUserContext } from '$lib/context';
  import { updateClubAvatar } from '$lib/stores/Clubs.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} clubId
   * @property {api.VideoChannel | null} clubChannel
   */

  /** @type {Props} */
  let { clubId, clubChannel } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let cropper = $state();

  /** @param {Event} e */
  async function submit(e) {
    e.preventDefault();

    const blob = await cropper.getCroppedBlob();
    const response = await updateClubAvatar(userCtx, clubId, blob);
  }
</script>

<h3>Logo</h3>
<ActorAvatar actor={clubChannel} />
<form onsubmit={submit}>
  <ImageCropper bind:this={cropper} width={192} height={192} />
  <button type="submit">Upload</button>
</form>
