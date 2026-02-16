<script>
  import { onMount } from 'svelte';
  import Combos from '$lib/components/combo/Combos.svelte';
  import { apiRequest } from '$lib/stats';

  /**
   * @typedef {Object} Props
   * @prop {number} userId
   * @prop {boolean} [showEditLink]
   */

  /** @type {Props}*/
  let { userId, showEditLink } = $props();

  /** @type {ComboInfo[]}*/
  let combos = $state([]);

  onMount(async () => {
    const result = await apiRequest(`/users/${userId}/combos`);
    // TODO: error handling
    /** @type {CombosResponse | undefined} */
    const response = await result.okResponse?.json();
    if (response && response.combos) {
      combos = response.combos;
    }
  });
</script>

<Combos {combos} {showEditLink} />
