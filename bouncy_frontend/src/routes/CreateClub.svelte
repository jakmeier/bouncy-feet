<script>
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import { getUserContext } from '$lib/context';
  import { t } from '$lib/i18n';
  import { createNewClub } from '$lib/stores/Clubs.svelte';

  const userCtx = getUserContext();

  async function submitForm(event) {
    event.preventDefault();

    const form = event.target;
    const title = form.title.value;
    const description = form.description.value;

    try {
      await createNewClub(userCtx, title, description);
      form.reset();
    } catch (err) {
      console.error(err);
    }
  }
</script>

{#if userCtx.isLoggedInToApi()}
  <LoginRequiredContent reason="">
    <form on:submit={submitForm}>
      <label for="title"> {$t('club.form-title')}: </label>
      <input type="text" name="title" required maxlength="64" />

      <label for="description"> {$t('club.form-description')}: </label>
      <textarea name="description" maxlength="1024" rows="5" cols="36" required
      ></textarea>

      <button type="submit">{$t('club.create-new-button')}</button>
    </form>
  </LoginRequiredContent>
{/if}

<style>
  form {
    display: grid;
    gap: 1rem;
    grid-template-columns: 1fr;
  }

  @media (min-width: 730px) {
    form {
      grid-template-columns: min-content auto;
    }
  }
</style>
