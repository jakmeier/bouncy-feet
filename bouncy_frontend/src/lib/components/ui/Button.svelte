<!-- @migration-task Error while migrating Svelte code: $$props is used together with named props in a way that cannot be automatically migrated. -->
<script>
  import { t } from '$lib/i18n';
  import Symbol from './Symbol.svelte';
  export let symbol = 'arrow_selector_tool';
  export let text = '';
  export let symbolClass = '';
  export let symbolSize = 42;

  $: hasSymbol = symbol != '';
</script>

<button class={$$props.class} on:click>
  {#if hasSymbol}
    <Symbol size={symbolSize} class={symbolClass || ''}>{symbol}</Symbol>
  {/if}
  <p class:smallText={hasSymbol}>
    {#if text !== ''}
      {$t(text)}
    {/if}
  </p>
</button>

<style>
  button p.smallText {
    padding: 0;
    margin: 0;
    font-size: var(--font-small);
  }
</style>
