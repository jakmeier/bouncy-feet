<script>
  import TabNavigation from './TabNavigation.svelte';
  import "../app.css";
</script>

<main>
  <slot />
</main>
<TabNavigation />

<style>
    main {
        margin: 5px;
    }

    @media (min-width: 730px) {
        main {
            max-width: 720px;
            margin: auto;
        }
    }
</style>
