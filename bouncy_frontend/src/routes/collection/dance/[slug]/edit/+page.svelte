<script>
  import { getContext } from 'svelte';
  import { page } from '$app/stores';
  import DanceEditor from '../../DanceEditor.svelte';
  import { t } from '$lib/i18n';
  import { DanceBuilder } from '$lib/instructor/bouncy_instructor';

  /** @type {import('./$types').PageData} */
  export let data;

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;

  const id = $page.params.slug;

  /** @type {DanceBuilder} */
  let danceBuilder = createDanceBuilder(id);

  /** @param {string} id */
  function createDanceBuilder(id) {
    // @ts-ignore
    const localDance = $localDances.find((dance) => dance.id === id);

    if (localDance) return localDance.asBuilder();

    // not a local dance, try if it is an official dance
    const officialDance = data.officialDances.find((dance) => dance.id === id);

    if (officialDance) {
      const builder = officialDance.asBuilder();
      const copy = $t('editor.dance-copy-postfix');
      builder.setId(`${id} (${copy})`);
      return builder;
    }

    console.error('dance not found', id);
    return new DanceBuilder(id);
  }
</script>

<DanceEditor availableSteps={data.allSteps} {danceBuilder} />
