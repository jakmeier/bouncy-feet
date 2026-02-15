<script>
  import * as api from '$lib/peertube-openapi';
  import { onMount } from 'svelte';
  import Symbol from '../Symbol.svelte';
  import { fetchVideo } from '$lib/peertube';

  /**
   * @typedef {Object} Props
   * @property {string | number} videoId -- id, uuid, or shortUuid
   * @property {(video: api.VideoDetails)=>void} onLoaded
   */

  /** @type Props */
  let { videoId, onLoaded } = $props();

  onMount(() => {
    fetchVideo(videoId).then(onLoaded);
  });
</script>

<div class="placeholder">
  <Symbol size={100} styleClass="rotating">refresh</Symbol>
</div>

<style>
  .placeholder {
    width: 300px;
    display: grid;
    place-items: center;
  }
</style>
