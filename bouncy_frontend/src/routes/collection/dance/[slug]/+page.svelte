<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import BackButton from '$lib/BackButton.svelte';

  // /** @type {import('./$types').PageData} */
  // export let data;
  const name = $page.params.slug;
</script>

<BackButton />

<h1>{name}</h1>

<style>
  h1 {
    text-align: center;
  }
</style>
