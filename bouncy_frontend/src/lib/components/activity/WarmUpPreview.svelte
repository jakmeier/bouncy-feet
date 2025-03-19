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
   * @property {()=>void} onBack
   */

  /** @type {Props} */
  let {
    videoUrl: video,
    description,
    audioControl,
    trackId,
    onDone,
    onBack,
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
  {onBack}
  {tracker}
  {video}
  {trackId}
  {difficulty}
  {energy}
/>
