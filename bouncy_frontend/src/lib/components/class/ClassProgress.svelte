<script>
  import { base } from '$app/paths';
  import { t } from '$lib/i18n';
  import { fade, slide } from 'svelte/transition';
  import Background from '../ui/sections/Background.svelte';
  import StandardPage from '../ui/StandardPage.svelte';
  import { onMount } from 'svelte';

  let { progress, onContinue } = $props();

  const elements = ['warmup', 'lesson', 'lesson', 'wrap'];
  let elementsShown = $state(0);
  let showButton = $state(false);

  function addElement() {
    elementsShown += 1;
    if (elementsShown <= progress) setTimeout(addElement, 600);
    else setTimeout(() => (showButton = true), 600);
  }

  onMount(() => {
    setTimeout(addElement, 500);
  });
</script>

<Background bgColor="var(--theme-main)" color="var(--theme-neutral-black)"
></Background>
<!-- TODO: come up with translated title -->
<StandardPage mainColor title="Daily Vibe">
  <div class="elements">
    {#each elements as element, i}
      {#if i < elementsShown}
        <div class="element" in:slide>
          <div class="element-number">
            {i + 1}
          </div>
          <!-- <div class="element-symbol">
            {#if element == 'warmup'}
              <Thermometer temperature={1}></Thermometer>
            {:else}
              <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
            {/if}
          </div> -->
          <div class="element-text">
            {element}
          </div>
          <!-- <div class="element-text">{element}</div> -->
          <div class="element-result-symbol">
            {#if i < progress}
              <img src="{base}/img/symbols/bf_check.svg" alt="done" />
            {:else}
              <img
                class="not-done"
                src="{base}/img/symbols/bf_eye.svg"
                alt="not done yet"
              />
            {/if}
          </div>
        </div>
      {/if}
    {/each}
  </div>

  {#if showButton}
    <div class="buttons" in:fade>
      <button class="wide" onclick={onContinue}>
        {$t('courses.lesson.next-button')}
      </button>
    </div>
  {/if}
</StandardPage>

<style>
  .elements {
    display: flex;
    flex-direction: column;
    gap: 3rem;
    margin: 2rem auto 3rem;
    max-width: 24rem;
  }
  .element {
    position: relative;
    display: grid;
    align-content: center;
    justify-content: space-around;
    text-align: center;
    grid-template-areas:
      'num result'
      'text text';
    padding: 2rem;
    background-color: var(--theme-neutral-dark);
    color: var(--theme-neutral-white);
    border-top-left-radius: 1rem;
    border-top-right-radius: 1rem;

    clip-path: polygon(0% 0%, 100% 0%, 100% 85%, 50% 100%, 0% 85%);
  }
  .element-number {
    font-size: 8rem;
    color: var(--theme-main);
    grid-area: num;
    align-self: center;
    justify-self: left;
  }
  /* .element-symbol {
    height: 6rem;
    grid-area: symbol;
    align-self: center;
  } */
  .element-result-symbol {
    width: 4rem;
    height: 8rem;
    grid-area: result;
    justify-self: right;
  }
  .element-result-symbol img {
    width: 100%;
    height: 100%;
  }
  .element-text {
    grid-area: text;
  }
  .not-done {
  }
  .buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  /* .element:last-child {
    border-radius: 1rem;
    clip-path: none;
  } */
</style>
