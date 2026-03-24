<script>
  // TODO: show more info about user, e.g. profile pic
  // TODO: pagination, search, sorting based on activity, etc etc

  import UnstyledButton from '$lib/components/ui/UnstyledButton.svelte';
  import { apiRequest } from '$lib/stats';
  import { goto } from '$app/navigation';
  import ActorAvatar from '../profile/ActorAvatar.svelte';

  /**
   * @typedef {Object} Props
   * @property {(user: PublicUserResponse)=>void} [onSelect]
   * @property {boolean} [verbose] -- always show handle + display name
   * (default: only show handle when no display name is set
   * @property {number} [pageSize] -- how many users to show at once
   * @property {UserSearchConfig} [searchConfig]
   */

  /** @type {Props} */
  let {
    onSelect = selectUser,
    verbose = false,
    pageSize = 20,
    searchConfig,
  } = $props();

  let loading = $state(true);
  let error = $state();

  /**
   * @type {PublicUserResponse[]}
   */
  let users = $state([]);
  let lastUsersUpdate = 0;
  // update users on config change - but at most once per second
  $effect(() => {
    const page = searchConfig?.page || 0;
    const searchTerm = searchConfig?.searchTerm || '';
    const showGuests = searchConfig?.showGuests || false;

    const doLoad = () => {
      lastUsersUpdate = Date.now();
      load(page, searchTerm, showGuests)
        .then((result) => {
          users = result;
        })
        .catch((e) => {
          error = e;
          loading = false;
        });
    };

    const elapsed = Date.now() - lastUsersUpdate;
    const delay = elapsed >= 1000 ? 0 : 1000 - elapsed;

    const timer = setTimeout(doLoad, delay);
    return () => clearTimeout(timer);
  });

  /**
   * @return {Promise<PublicUserResponse[]>}
   * @param {number} loadPage
   * @param {string} searchTerm
   * @param {boolean} showGuests
   */
  async function load(loadPage, searchTerm, showGuests) {
    const query = new URLSearchParams();
    if (searchTerm && searchTerm.length >= 2 && searchTerm.length <= 100) {
      query.append('name_fragment', searchTerm);
    }
    query.append('include_guests', showGuests.toString());
    query.append('offset', (loadPage * pageSize).toString());
    query.append('limit', pageSize.toString());

    loading = true;
    const res = await apiRequest(`/users?${query}`);
    let result = await res.okResponse?.json();
    loading = false;
    return result.users;
  }

  /** @param { PublicUserResponse} user */
  function selectUser(user) {
    goto(`/users/${user.id}`);
  }
</script>

{#if loading}
  <p>Loading...</p>
{:else if error}
  <p style="color:red">{error.message}</p>
{:else}
  <ul>
    {#each users as user}
      <li>
        <UnstyledButton onClick={() => onSelect(user)}>
          <div class="pic">
            <!-- TODO: better user default avatar -->
            <ActorAvatar url={user.small_avatar}></ActorAvatar>
          </div>
          <div>
            {#if user.display_name || user.peertube_handle}
              {#if verbose && user.display_name && user.peertube_handle}
                {user.display_name}(@{user.peertube_handle})
              {:else if user.display_name}
                {user.display_name}
              {:else if user.peertube_handle}
                @{user.peertube_handle}
              {/if}
            {:else}
              Anonymous #{user.id}
            {/if}
          </div>
        </UnstyledButton>
      </li>
    {/each}
  </ul>
{/if}

<style>
  ul {
    color: var(--theme-neutral-dark);
    width: 100%;
    overflow-y: auto;
  }

  li {
    display: flex;
    gap: 0.5rem;
    text-align: left;
    align-items: center;
    background-color: var(--theme-main-light);
    padding: 0.5rem;
    border-radius: 0.5rem;
    border-top: var(--theme-main) solid 1px;
  }

  li:last-child {
    border-bottom: var(--theme-main) solid 1px;
  }

  .pic {
    width: 32px;
    height: 32px;
  }
</style>
