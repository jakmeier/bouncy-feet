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

  let apiUserLoading = $state(true);
  let userIdLoading = $state(false);
  let fullUserLoading = $state(loadFullUser);

  $effect(() => {
    loading = userIdLoading || apiUserLoading || fullUserLoading;
  });
</script>

{#if loadApiUser && !apiUser}
  <ApiUserLoader bind:apiUser bind:loading={apiUserLoading} />
{/if}

{#if loadApiUser && !apiUserLoading}
  <UserIdLoader bind:loading={userIdLoading} {setError} />
{/if}

{#if !userIdLoading && apiUser && !fullUser}
  <FullUserLoader bind:fullUser bind:loading={fullUserLoading} {apiUser} />
{/if}
