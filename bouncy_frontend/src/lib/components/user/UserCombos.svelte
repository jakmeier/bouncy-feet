<script>
  import { onMount } from 'svelte';
  import Combos from '$lib/components/combo/Combos.svelte';
  import { apiRequest } from '$lib/stats';

  /**
   * @typedef {Object} Props
   * @prop {number} [userId] -- required if apiUser is not set
   * @prop {ApiUser} [apiUser] -- only set if private combos should be shown
   * @prop {boolean} [showEditLink]
   * @prop {boolean} [hasContent] -- bindable
   */

  /** @type {Props}*/
  let { userId, showEditLink, apiUser, hasContent = $bindable() } = $props();

  /** @type {ComboInfo[]}*/
  let combos = $state([]);

  onMount(async () => {
    let result;
    if (apiUser) {
      result = await apiUser.authenticatedGet(`/user/combos`);
    } else {
      result = await apiRequest(`/users/${userId}/combos`);
    }

    if (result.error) {
      console.warn('Failed loading combos', result);
      hasContent = false;
      return;
    }

    /** @type {CombosResponse | undefined} */
    const response = await result.okResponse?.json();
    if (response && response.combos) {
      combos = response.combos;
    }
    if (combos.length > 0) {
      hasContent = true;
    } else {
      hasContent = false;
    }
  });
</script>

<Combos {combos} {showEditLink} {apiUser} />
