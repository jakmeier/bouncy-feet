<script>
  import { getUserContext } from '$lib/context';
  import { createNewClub, updateClubAvatar } from '$lib/stores/Clubs.svelte';
  import ClubForm from './ClubForm.svelte';

  const userCtx = getUserContext();
  let form = $state();

  /** @type {EditableClubDetails} */
  let details = $state({
    description: '',
    url: '',
  });
  let name = $state('');

  /** @param {Blob|null} blob */
  async function submit(blob) {
    const { description, url } = details;

    try {
      const club = await createNewClub(userCtx, name, description, url);
      if (club && blob) {
        await updateClubAvatar(userCtx, club.id, blob);
      }

      // clean up form
      details = {
        description: '',
        url: '',
      };
      form.reset();
    } catch (err) {
      console.error(err);
    }
  }
</script>

<ClubForm bind:this={form} bind:details bind:name onSubmit={submit}></ClubForm>
