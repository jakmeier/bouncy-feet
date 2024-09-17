<script>
  import { t } from '$lib/i18n';
  import Symbol from './Symbol.svelte';
  export let symbol = 'arrow_selector_tool';
  export let text = '';
  export let symbolClass = '';
  export let symbolSize = 42;
</script>

<button class={$$props.class} on:click>
  <Symbol size={symbolSize} class={symbolClass || ''}>{symbol}</Symbol>
  <p>{$t(text)}</p>
</button>

<style>
  button {
    height: 80px;
    width: 120px;
  }

  button.big {
    width: 152px;
    height: 80px;
    margin: 10px;
  }

  button.wide {
    margin: 5px auto;
    width: 90%;
  }

  button.locked {
    background-color: var(--theme-neutral-gray);
    color: var(--theme-neutral-dark);
    opacity: 0.5;
    cursor: not-allowed;
  }

  button.reset {
    width: 80px;
    height: 30px;
    background-color: var(--theme-accent-light);
    color: var(--theme-neutral-dark);
    text-align: center;
    padding: 0;
  }

  button.blue {
    color: var(--theme-main);
  }

  button.margin15 {
    margin: 15px;
  }

  @media (max-width: 360px) {
    /* Three buttons must fit next to each other, including margin */
    button {
      width: 75px;
      height: fit-content;
    }
  }
</style>
