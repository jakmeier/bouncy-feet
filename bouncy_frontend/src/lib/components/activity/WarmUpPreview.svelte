<script>
  import { t } from '$lib/i18n';
  import { getContext, onDestroy, onMount } from 'svelte';

  import ActivityPreview from './ActivityPreview.svelte';

  /**
   * @typedef {Object} Props
   * @property {string} videoUrl
   * @property {string} description
   * @property {boolean} audioControl
   * @property {string} trackId
   * @property {()=>void} onDone
   */

  /** @type {Props} */
  let {
    videoUrl: video,
    description,
    audioControl,
    trackId,
    onDone,
  } = $props();

  let { tracker } = getContext('tracker');

  // TODO: Get this from somewhere
  const difficulty = 1;
  const energy = 3;

  const title = $t('record.warmup-preview-title');
</script>

<ActivityPreview
  {title}
  {description}
  {onDone}
  {tracker}
  {video}
  {trackId}
  {difficulty}
  {energy}
/>
