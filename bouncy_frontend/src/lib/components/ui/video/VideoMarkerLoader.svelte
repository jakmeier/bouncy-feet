<script>
  import { onMount } from 'svelte';

  /**
   * @typedef {Object} Props
   * @property {number} comboId
   * @property {(markers: VideoMarker[])=>void} onLoaded
   * @property {ApiUser} apiUser -- TODO: make this optional for public viewing
   */

  /** @type Props */
  let { comboId, onLoaded, apiUser } = $props();

  onMount(async () => {
    const result = await apiUser.authenticatedGet(
      `/combos/${comboId}/timestamp`
    );
    if (result.okResponse) {
      /** @type {ComboTimestampInfos} */
      const markersResult = await result.okResponse.json();
      /** @type {VideoMarker[]} */
      const markers = markersResult.ms.map((ms) => ({
        time: ms,
        icon: '',
        label: '',
      }));
      if (markers) {
        onLoaded(markers);
      } else {
        console.error('No markers result', markers);
      }
    } else {
      console.error(
        'Failed loading timestamps',
        result.error,
        result.errorBody
      );
    }
  });
</script>
