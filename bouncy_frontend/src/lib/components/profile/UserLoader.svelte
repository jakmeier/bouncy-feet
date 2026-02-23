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
    loadFullUser = false,
    setError,
  } = $props();

  let apiUserLoading = $state(true);
  let userIdLoading = $state(true);
  let fullUserLoading = $state(loadFullUser);

  $effect(() => {
    loading = userIdLoading || apiUserLoading || (apiUser && fullUserLoading);
  });
</script>

<ApiUserLoader bind:apiUser bind:loading={apiUserLoading} />

{#if !apiUserLoading}
  <UserIdLoader bind:loading={userIdLoading} {setError} />
{/if}

{#if apiUser && loadFullUser}
  <FullUserLoader bind:fullUser bind:loading={fullUserLoading} {apiUser} />
{/if}
