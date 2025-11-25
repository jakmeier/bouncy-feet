<script>
  import { getClubsContext } from '$lib/stores/Clubs.svelte';

  const { clubsData } = getClubsContext();
</script>

{#each clubsData.mine as club}
  <a href="./club/{club.id}">
    <div class="club card">
      <div class="logo">
        <div class="title" style="color: {club.style.coloring.headColor}">
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
              {club.stats.members} members
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
