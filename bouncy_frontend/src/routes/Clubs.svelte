<script>
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import { getUserContext } from '$lib/context';
  import { createNewClub, getClubsContext } from '$lib/stores/Clubs.svelte';

  const { clubsData } = getClubsContext();
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

<!-- WIP -->
<LoginRequiredContent reason="testing">
  <form on:submit={submitForm}>
    <label>
      Title:
      <input type="text" name="title" required maxlength="64" />
    </label>

    <label>
      Description:
      <textarea name="description" maxlength="1024" required></textarea>
    </label>

    <button type="submit">Create club</button>
  </form>
</LoginRequiredContent>

{#each clubsData.mine as club}
  <a href="./club/{club.id}">
    <div class="club card">
      <div class="logo">
        <!-- style="color: {club.style.coloring.headColor}" -->
        <div class="title">
          {club.name}
        </div>
      </div>
      <div class="text">
        <div class="description">
          <!-- <FormattedText
          text={club.description}
          color={club.style.coloring.headColor}
        ></FormattedText> -->
          <div class="stats">
            <div>
              {club.stats?.members || '?'} members
            </div>
            <div>5 new videos</div>
          </div>
        </div>
      </div>
    </div>
  </a>
{/each}

<style>
  .club {
    display: grid;
    margin-top: 1rem;
    margin-bottom: 0rem;
    position: relative;
    gap: 0.5rem;
  }

  .description {
    font-size: var(--font-normal);
  }

  .logo {
    padding: 0.5rem;
    background-color: var(--theme-neutral-black);
    display: grid;
    justify-content: center;
    align-content: center;
    border-radius: 1rem;
    background-color: var(--theme-neutral-almost-black);
  }

  .stats {
    color: var(--theme-neutral-darker-gray);
    font-size: small;
    display: flex;
    flex-wrap: wrap;
    justify-content: space-evenly;
  }

  .card {
    background-color: var(--theme-neutral-almost-black);
    padding: 1rem;
    border-radius: 1rem;
  }
</style>
