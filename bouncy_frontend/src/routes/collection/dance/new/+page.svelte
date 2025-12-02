<script>
  import { t } from '$lib/i18n.js';
  import { DanceBuilder } from '$lib/instructor/bouncy_instructor';
  import DanceEditor from '../DanceEditor.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  let danceName = $t('editor.new-dance.default-dance-name');
  /** @type {DanceBuilder} */
  let danceBuilder = new DanceBuilder(danceName);

  const uniqueNameSteps = data.lookupSteps({ uniqueNames: true });
</script>

<DanceEditor availableSteps={uniqueNameSteps} {danceBuilder} />
