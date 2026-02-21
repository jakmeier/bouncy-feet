<script>
  import { ONBOARDING_STATE } from '$lib/onboarding';
  import { ApiUser } from '$lib/stores/ApiUser.svelte';
  import { getUserContext } from '$lib/stores/context';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {ApiUser|undefined} apiUser --bindable
   * @property {boolean} [loading] --bindable
   */

  /** @type {Props} */
  let { apiUser = $bindable(), loading = $bindable(true) } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  onMount(async () => {
    if (!apiUser) {
      apiUser = await userCtx.maybeLoadApiUser();
    }
    if (apiUser) {
      // TODO: Maybe check first if a sync is needed?
      // await apiUser.syncKvWithServer();
      userCtx.user.publicName = apiUser.meta['publicName'];
    }

    loading = false;
  });
</script>
