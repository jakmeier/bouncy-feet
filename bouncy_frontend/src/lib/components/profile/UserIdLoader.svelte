<script>
  import { onMount } from 'svelte';
  import { apiRequest } from '$lib/stats';
  import { getUserContext } from '$lib/stores/context';

  /**
   * @typedef {Object} Props
   * @property {boolean} [loading] --bindable
   * @property {(err: BfError)=>void} setError
   */

  /** @type {Props} */
  let { loading = $bindable(true), setError } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  /**
   * @returns {Promise<import('$lib/stats').ApiError | undefined>}
   */
  async function refreshUserId() {
    let apiResponse;
    if (!userCtx.apiUser) {
      // may work with an active session for non-guest users
      apiResponse = await apiRequest('/user');
    } else {
      // works for guest and non-guest users
      apiResponse = await userCtx.apiUser.authenticatedGet('/user');
    }

    if (apiResponse.error) {
      return apiResponse.error;
    }

    let response = apiResponse.okResponse;
    try {
      /** @type {PrivateUserInfoResponse} */
      const userInfo = await response?.json();
      if (!userInfo || userInfo.sub === undefined) {
        throw new Error(`missing sub in response: ${JSON.stringify(userInfo)}`);
      }

      // note: Sub may be null, this means the user is a guest.
      if (!userInfo.sub && userCtx.user.openid) {
        console.warn(
          'Client has a oidc subject but the server thinks this is a guest. Dropping previously known oidc subject.'
        );
      }
      userCtx.user.openid = userInfo.sub;
      userCtx.user.apiId = userInfo.id;
    } catch (errResponse) {
      console.warn('Failed reading user info');
    }
  }

  onMount(async () => {
    if (!userCtx.user.openid || userCtx.user.apiId === undefined) {
      let err = await refreshUserId();
      if (err) {
        setError({
          title: 'User id fetching failed',
          description: err,
        });
      }
    }
    loading = false;
  });
</script>
