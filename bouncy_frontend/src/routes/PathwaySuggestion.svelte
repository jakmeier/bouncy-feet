<script>
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { t } from '$lib/i18n';

  import Step from './collection/Step.svelte';

  let {
    size,
    step,
    title,
    fitness,
    xp,
    min,
    coach
  } = $props();

  let selected = $state(false);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="outer"
  onclick={() => {
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
    border-radius: 50dvh;
    font-size: var(--font-normal);
    padding: 2px;
    margin: 2px;
  }

  a {
    text-decoration: none;
  }
</style>
