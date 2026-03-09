<script>
  import { apiRequest } from '$lib/stats';
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {number} comboId
   * @property {(beats: Beat[])=>void} onLoaded
   * @property {ApiUser} [apiUser]
   */

  /** @type Props */
  let { comboId, onLoaded, apiUser } = $props();

  onMount(async () => {
    let result;
    if (apiUser) {
      result = await apiUser.authenticatedGet(`/user/combos/${comboId}/beat`);
    } else {
      result = await apiRequest(`/combos/${comboId}/beat`);
    }
    if (result.okResponse) {
      /** @type {Beat[]} */
      const beats = await result.okResponse.json();
      if (beats) {
        onLoaded(beats);
      } else {
        console.error('No result', beats);
      }
    } else {
      console.error('Failed loading beats', result.error, result.errorBody);
    }
  });
</script>
