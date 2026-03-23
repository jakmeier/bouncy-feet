<script>
  import { onMount } from 'svelte';

  // Set public display name on the user to the value of the handle. This is
  // called when a user registers directly, without creating a guest user
  // first, which means there is no guest name.

  /**
   * @typedef {Object} Props
   * @property {ApiUser} apiUser
   * @property {number} userId
   */

  /** @type {Props} */
  let { apiUser, userId } = $props();

  /** @return {Promise<PublicUserResponse>} */
  async function loadMyUser() {
    const res = await apiUser.authenticatedGet(`/users/${userId}`);
    return await res.okResponse?.json();
  }

  onMount(async () => {
    const info = await loadMyUser();
    console.assert(!info.display_name, info.display_name, 'should be empty');
    let newName = info.peertube_handle;
    if (newName.startsWith('@')) {
      newName = newName.substring(1);
    }
    await apiUser.setUserMeta('publicName', newName);
  });
</script>
