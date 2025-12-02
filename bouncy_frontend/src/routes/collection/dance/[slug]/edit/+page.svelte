<script>
  import { getContext } from 'svelte';
  import { page } from '$app/state';
  import DanceEditor from '../../DanceEditor.svelte';
  import { t } from '$lib/i18n';
  import {
    DanceBuilder,
    danceBuilderFromDance,
  } from '$lib/instructor/bouncy_instructor';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;
  const fileBuilder = localCollection.danceBuilder;

  const id = page.params.slug;

  /** @type {DanceBuilder} */
  let danceBuilder = createDanceBuilder(id);

  /** @param {string} id */
  function createDanceBuilder(id) {
    // @ts-ignore
    const localDance = $localDances.find((dance) => dance.id === id);

    if (localDance) return $fileBuilder.danceBuilder(localDance.id);

    // not a local dance, try if it is an official dance
    const officialDance = data.officialDances.find((dance) => dance.id === id);

    if (officialDance) {
      const builder = danceBuilderFromDance(officialDance.id);
      const copy = $t('editor.dance-copy-postfix');
      builder.setId(`${id} (${copy})`);
      return builder;
    }

    console.error('dance not found', id);
    return new DanceBuilder(id);
  }
</script>

<DanceEditor
  availableSteps={data.lookupSteps({ uniqueNames: false })}
  {danceBuilder}
/>
