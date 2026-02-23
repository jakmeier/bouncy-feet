<script>
  import { getUserContext } from '$lib/stores/context';
  import { initFullUser } from '$lib/stores/FullUser.svelte';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {ApiUser} apiUser
   * @property {FullUser|undefined} fullUser --bindable
   * @property {boolean} [loading] --bindable
   */

  /** @type {Props} */
  let { apiUser, fullUser = $bindable(), loading = $bindable(true) } = $props();

  /** @type {UserContextData} */
  const userCtx = getUserContext();

  let fullUserLock = false;
  /**
   * @returns {Promise<FullUser>}
   */
  async function ensureFullUser() {
    if (!userCtx.fullUser) {
      if (!fullUserLock) {
        fullUserLock = true;
        userCtx.fullUser = await initFullUser(apiUser);
      } else {
        while (!userCtx.fullUser) {
          await new Promise((resolve) => setTimeout(resolve, 100));
        }
      }
    }
    return userCtx.fullUser;
  }

  onMount(() => {
    ensureFullUser().then((fullUser) => {
      // TODO
      // clearErrors();
      const _token = fullUser.peerTubeToken();
      // // TODO: prevent this from firing too often
      // if (!fullUser.isLoggedInToApi()) {
      //   try {
      //     const _token = await fullUser.peerTubeToken();
      //   } catch (e) {
      //     console.debug('failed to refresh PeerTube token', e);
      //   }
      // }
    });

    loading = false;
  });
</script>
