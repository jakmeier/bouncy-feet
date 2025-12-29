<script>
  import * as api from '$lib/peertube-openapi';
  import { getUserContext } from '$lib/context';
  import { updateClub, updateClubAvatar } from '$lib/stores/Clubs.svelte';
  import ClubForm from './ClubForm.svelte';

  /**
   * @typedef {Object} Props
   * @property {Club} club
   * @property {ClubDetailsResponse} clubDetails
   * @property {api.VideoChannel | undefined} clubChannel
   * @property {()=>void} [onUpdateSuccess]
   */

  /** @type {Props} */
  let { club, clubDetails, clubChannel, onUpdateSuccess = () => {} } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let form = $state();

  /** @type {EditableClubDetails} */
  let details = $state({
    description: club.description,
    url: clubDetails.web_link,
  });

  /** @param {Blob|null} blob */
  async function submit(blob) {
    const { description, url } = details;

    try {
      if (club.description !== description || clubDetails.web_link !== url) {
        await updateClub(userCtx, club.id, {
          description,
          url,
        });
      }

      if (blob) {
        await updateClubAvatar(userCtx, club.id, blob);
      }

      onUpdateSuccess();
    } catch (err) {
      console.error(err);
    }
  }
</script>

<ClubForm
  bind:this={form}
  bind:details
  {club}
  {clubChannel}
  onSubmit={submit}
/>
