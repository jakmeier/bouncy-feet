<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import Select from 'svelte-select';
  import { dynamicCounter } from '$lib/timer';
  import { features } from '$lib/stores/FeatureSelection';
  import { browser } from '$app/environment';
  import Info from '$lib/components/ui/Info.svelte';
  import { getContext } from 'svelte';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Step from '../../../../collection/Step.svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm } from '$lib/stores/Beat';
  import TrackerPreview from '$lib/components/avatar/TrackerPreview.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const user = getContext('user').store;
  const name = page.params.stepName;
  const variations = data.lookupSteps({
    uniqueNames: false,
    stepName: name,
  });
  const selectItems = variations.map((step) => {
    return { value: step, label: $t(`step.variation.${step.variation}`) };
  });

  let selected = $state(selectItems[0]);

  const { getCourse } = getContext('courses');
  //   TODO: map courses to steps
  //   const coach = $derived(coachData(coachId));
  //   const courses = $derived(coach.courseIds.map(getCourse));
  /**
   * @type {import('bouncy_instructor').Course[]}
   */
  const courses = [];
</script>

<Header title={name} />

<AnimatedStep step={selected.value} size={200} backgroundColor="transparent"
></AnimatedStep>

<!-- TODO: Video -->
<!-- TODO: Counts and poses -->

<!-- TODO: style slider -->
<label>
  {$t('collection.step.speed')}
  <input type="number" bind:value={$bpm} min="15" max="200" class="number" />
  <input type="range" bind:value={$bpm} min="15" max="200" class="range" />
</label>

{#if selectItems.length > 1}
  <div class="label">
    {$t('collection.step.variation')}
    <Select
      bind:value={selected}
      items={selectItems}
      showChevron={true}
      clearable={false}
      searchable={false}
      --background="var(--theme-neutral-light)"
      --selected-item-color="var(--theme-neutral-dark)"
      --item-hover-bg="var(--theme-main)"
      --item-hover-color="var(--theme-neutral-light)"
      --item-active-background="var(--theme-accent)"
      --item-is-active-bg="var(--theme-neutral-white)"
      --item-is-active-color="var(--theme-neutral-dark)"
      --border="1px solid var(--theme-neutral-dark)"
      --border-hover="1.5px solid var(--theme-main)"
      --border-focused="1.5px solid var(--theme-main)"
      --margin="10px auto"
      --padding="10px"
      --font-size="var(--font-normal)"
    />
  </div>

  <h2>{$t('collection.courses-subtitle')}</h2>

  <!-- TODO: fix display of courses + link for courses -->
  {#each courses as course}
    <p>{course.name}</p>
    <div class="ol">
      {#each course.lessons as lesson, index}
        <div class="lesson-outer">
          <div class="corner-marked2">
            <div class="corner-marked">
              <div class="lesson-inner">
                <a href="../../courses/{course.id}/exercise/{index}">
                  <div class="preview">
                    <TrackerPreview
                      tracker={course.tracker(index)}
                      size={150}
                      backgroundColor="transparent"
                    ></TrackerPreview>
                  </div>
                  <div class="lesson-name">{lesson.name}</div>
                </a>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <p>{$t('collection.no-courses')}</p>
  {/each}
{/if}

<style>
  label,
  .label {
    display: grid;
    justify-items: center;
    margin: 10px auto;
    max-width: 300px;
    background-color: var(--theme-main);
    color: var(--theme-neutral-black);
    border-radius: 10px;
    padding: 10px;
  }
  .ol {
    display: flex;
    overflow: scroll;
    padding-bottom: 1rem;
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }
  .lesson-outer {
    padding: 0.5rem;
    max-width: min(300px, 68vw);
    margin: 0.5rem;
    word-wrap: break-word;
  }
  .preview {
    margin: 0 0.5rem;
    height: 100%;
  }
  .lesson-name {
    text-align: center;
    padding-bottom: 1rem;
  }
</style>
