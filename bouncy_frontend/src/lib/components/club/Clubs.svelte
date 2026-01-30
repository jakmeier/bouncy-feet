<script>
  import { getClubsContext } from '$lib/stores/context';
  import ActorAvatar from '../profile/ActorAvatar.svelte';
  import FormattedText from '../ui/FormattedText.svelte';

  const { clubsData } = getClubsContext();
</script>

{#each clubsData.mine as club}
  <a href="/club/{club.id}">
    <div class="club card">
      <ActorAvatar url={club.avatar} targetWidth={120}></ActorAvatar>
      <!-- style="color: {club.style.coloring.headColor}" -->
      <div class="text">
        <div class="title">
          {club.name}
        </div>
        <div class="description">
          <FormattedText
            text={club.description}
            color={club.style.coloring.headColor}
          ></FormattedText>
        </div>
      </div>
    </div>
  </a>
{/each}

<style>
  .club {
    display: flex;
    margin-top: 1rem;
    margin-bottom: 0rem;
    position: relative;
    gap: 0.5rem;
  }

  .text {
    display: grid;
    width: 100%;
    grid-template-columns: 1fr 1fr;
    justify-items: center;
    align-items: center;
  }

  .description {
    font-size: var(--font-normal);
    color: var(--theme-neutral-white);
  }

  .title {
    padding: 0.5rem;
    display: grid;
    justify-content: center;
    align-content: center;
  }

  .card {
    background-color: var(--theme-neutral-almost-black);
    padding: 1rem;
    border-radius: 1rem;
  }
</style>
