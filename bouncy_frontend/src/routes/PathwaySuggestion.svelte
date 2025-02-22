<script>
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { t } from '$lib/i18n';

  import Step from './collection/Step.svelte';

  export let size;
  export let step;
  export let title;
  export let fitness;
  export let xp;
  export let min;
  export let coach;

  let selected = false;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="outer"
  on:click={() => {
    selected = !selected;
  }}
>
  <div>{$t(title)}</div>
  {#if selected}
    <a href="./coach/{coach}">
      <AnimatedStep {size} {step} />
      <div class="bubbles">
        <div>+{fitness} Fitness</div>
        <div>+{xp} XP</div>
        <div>{min}min</div>
      </div>
    </a>
  {:else}
    <Step {size} {step}/>
  {/if}
</div>

<style>
  .bubbles div {
    background-color: var(--theme-accent);
    color: var(--theme-neutral-white);
    border-radius: 50vh;
    font-size: var(--font-normal);
    padding: 2px;
    margin: 2px;
  }

  a {
    text-decoration: none;
  }
</style>
