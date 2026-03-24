<script>
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import UserList from '$lib/components/user/UserList.svelte';
  import UserListControl from '$lib/components/user/UserListControl.svelte';
  import UserListPagination from '$lib/components/user/UserListPagination.svelte';

  /** @type {UserSearchConfig} */
  let searchConfig = $state({
    searchTerm: '',
    page: 0,
    showGuests: false,
  });
</script>

<StandardPage mainColor backButton title="home.users-title">
  <UserListControl bind:searchConfig></UserListControl>
  <UserList {searchConfig}></UserList>
  <UserListPagination bind:searchConfig></UserListPagination>
</StandardPage>
