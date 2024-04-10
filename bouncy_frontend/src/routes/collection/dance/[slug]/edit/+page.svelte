<script>
  import { getContext } from 'svelte';
  import { page } from '$app/stores';
  import DanceEditor from '../../DanceEditor.svelte';

  /** @type {import('./$types').PageData} */
  export let data;

  const localCollection = getContext('localCollection');
  const localDances = localCollection.dances;

  const id = $page.params.slug;
  /** @type {import('$lib/instructor/bouncy_instructor').DanceInfo} */
  const dance = $localDances.find(
    (
      /** @type {import('$lib/instructor/bouncy_instructor').DanceInfo} */ dance
    ) => dance.id === id
  );
  /** @type {import('$lib/instructor/bouncy_instructor').DanceBuilder} */
  let danceBuilder = dance.asBuilder();
</script>

<DanceEditor availableSteps={data.uniqueNameSteps} {danceBuilder} />
