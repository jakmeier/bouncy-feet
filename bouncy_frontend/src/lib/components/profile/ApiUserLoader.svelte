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

  let apiUserLock = false;
  /** @returns {Promise<ApiUser|undefined>} */
  async function maybeLoadApiUser() {
    if (!userCtx.apiUser) {
      if (!apiUserLock) {
        apiUserLock = true;
        const tmpApiUser = await ApiUser.initApiUser(false, userCtx);

        // make the first use of the API user to see if it has a valid session
        // (returning users can restore a client session but won't be logged in)
        if (tmpApiUser) {
          let err = await tmpApiUser.syncKvWithServer();
          if (err) {
            apiUserLock = false;
            return undefined;
          }
        }

        userCtx.apiUser = tmpApiUser;
        apiUserLock = false;

        if (userCtx.apiUser) {
          // migration from old to new way of storing user meta in local storage
          await userCtx.apiUser.clientSession.migrateFromFirstMetaStorage(
            userCtx.apiUser.kvSync
          );
        }
      } else {
        while (!userCtx.apiUser) {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      }
    }
    return userCtx.apiUser;
  }

  // TODO: this is a weird place fr this code
  export async function createGuestUser() {
    console.assert(
      !userCtx.apiUser,
      'creating a guest while an account already exists'
    );

    while (apiUserLock) {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
    apiUserLock = true;
    const apiUser = await ApiUser.initApiUser(true, userCtx);
    console.assert(!!apiUser, 'creating a new guest API user failed');
    userCtx.apiUser = apiUser;
    apiUserLock = false;

    // note: apiUser should have an active session now, as it was just created.
    if (apiUser) {
      apiUser.kvSync.setStringValue(
        'onboarding',
        ONBOARDING_STATE.FIRST_VISIT,
        new Date()
      );
      apiUser.kvSync.setStringValue(
        'publicName',
        apiUser.userCtx.user.publicName,
        new Date()
      );
    }
  }

  onMount(async () => {
    if (!apiUser) {
      apiUser = await maybeLoadApiUser();
    }
    if (apiUser) {
      // TODO: Maybe check first if a sync is needed?
      // await apiUser.syncKvWithServer();
      userCtx.user.publicName = apiUser.meta['publicName'];
    }

    loading = false;
  });
</script>
