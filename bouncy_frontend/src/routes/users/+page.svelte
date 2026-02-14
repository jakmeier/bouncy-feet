<script>
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import UserList from '$lib/components/user/UserList.svelte';
</script>

<StandardPage mainColor backButton title="home.users-title">
  <UserList></UserList>
</StandardPage>
