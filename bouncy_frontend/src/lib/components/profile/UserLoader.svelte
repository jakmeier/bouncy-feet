<script>
  import ApiUserLoader from './ApiUserLoader.svelte';
  import FullUserLoader from './FullUserLoader.svelte';
  import UserIdLoader from './UserIdLoader.svelte';

  /**
   * @typedef {Object} Props
   * @property {ApiUser} [apiUser] --bindable
   * @property {FullUser} [fullUser] --bindable
   * @property {boolean} [loading] --bindable
   * @property {boolean} [loadApiUser]
   * @property {boolean} [loadFullUser]
   * @property {(err: BfError)=>void} setError
   */

  /** @type {Props} */
  let {
    apiUser = $bindable(),
    fullUser = $bindable(),
    loading = $bindable(true),
    loadApiUser = false,
    loadFullUser = false,
    setError,
  } = $props();

  let userIdLoading = $state(true);
  let apiUserLoading = $state(loadApiUser);
  let fullUserLoading = $state(loadFullUser);

  let apiUserLoader = $state();

  $effect(() => {
    loading = userIdLoading || apiUserLoading || fullUserLoading;
  });

  export async function createGuestUser() {
    await apiUserLoader.createGuestUser();
  }
</script>

<UserIdLoader bind:loading={userIdLoading} {setError} />

{#if !userIdLoading && loadApiUser && !apiUser}
  <ApiUserLoader
    bind:this={apiUserLoader}
    bind:apiUser
    bind:loading={apiUserLoading}
  />
{/if}

{#if !userIdLoading && apiUser && !fullUser}
  <FullUserLoader bind:fullUser bind:loading={fullUserLoading} {apiUser} />
{/if}
