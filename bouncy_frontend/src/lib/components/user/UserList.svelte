<script>
  // TODO: show more info about user, e.g. profile pic
  // TODO: pagination, search, sorting based on activity, etc etc

  import { onMount } from 'svelte';
  import UnstyledButton from '$lib/components/ui/UnstyledButton.svelte';
  import { apiRequest } from '$lib/stats';
  import { goto } from '$app/navigation';

  /**
   * @typedef {Object} Props
   * @property {(user: PublicUserResponse)=>void} [onSelect]
   */

  /** @type {Props} */
  let { onSelect = selectUser } = $props();

  async function load() {
    const res = await apiRequest('/users');
    let result = await res.okResponse?.json();
    return result?.users;
  }

  /** @param { PublicUserResponse} user */
  function selectUser(user) {
    goto(`/users/${user.id}`);
  }

  onMount(load);
</script>

{#await load()}
  <p>Loading...</p>
{:then users}
  <ul>
    {#each users as user}
      <li>
        <UnstyledButton onClick={() => onSelect(user)}>
          <div>{user.display_name}</div>
        </UnstyledButton>
      </li>
    {/each}
  </ul>
{:catch error}
  <p style="color:red">{error.message}</p>
{/await}

<style>
  ul {
    color: var(--theme-neutral-dark);
    width: 100%;
    max-height: 40vh;
    overflow-y: auto;
  }

  li {
    text-align: left;
    background-color: var(--theme-main-light);
    padding: 0.5rem;
    border-radius: 0.5rem;
    border-top: var(--theme-main) solid 1px;
  }

  li:last-child {
    border-bottom: var(--theme-main) solid 1px;
  }
</style>
