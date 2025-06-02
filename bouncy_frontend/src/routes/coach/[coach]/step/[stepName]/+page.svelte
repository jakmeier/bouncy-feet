<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext, onMount } from 'svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm, beatCounter } from '$lib/stores/Beat';
  import { coaches } from '$lib/coach';
  import Video from '$lib/components/ui/Video.svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import DanceCounts from '$lib/components/DanceCounts.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const name = page.params.stepName;
  const coachId = page.params.coach;

  const { getCourse } = getContext('courses');

  const variations = data.lookupSteps({
    uniqueNames: false,
    stepName: name,
  });
  let step = variations[0];

  /**
   * @param {string} coachId
   */
  function coachData(coachId) {
    const coachData = coaches.find((c) => c.name === coachId);
    if (coachData) {
      return coachData;
    } else {
      return coaches[0];
    }
  }

  const coach = $derived(coachData(coachId));
  const coachStep = $derived(coach.steps[step.id]);
  const video = $derived(coachStep.video);

  /**
   * @type {import('bouncy_instructor').Course[]}
   */
  const courses = $derived(coachStep.courses.map(getCourse));

  onMount(() => {
    $bpm = 45;
  });
</script>

<LightBackground />

<Header title={name} />

<div class="video-wrapper">
  {#if video && video.length > 0}
    <Video path={`${video}`}></Video>
  {/if}
</div>

<DanceCounts steps={[step]} markedPoseIndex={$beatCounter} />

<AnimatedStep {step} size={200} backgroundColor="transparent"></AnimatedStep>

<!-- TODO(Tanja): style slider  -->
<label>
  {$t('collection.step.speed')}
  <input type="number" bind:value={$bpm} min="15" max="200" class="number" />
  <input type="range" bind:value={$bpm} min="15" max="200" class="range" />
</label>

<DarkSection>
  <h2>{$t('collection.courses-subtitle')}</h2>

  {#each courses as course}
    <div class="course">
      <a href="../../../../courses/{course.id}">
        <button>{course.name}</button>
      </a>
      <div class="ol">
        {#each course.lessons as lesson, index}
          <a href="../../../../courses/{course.id}/exercise/{index}">
            <!-- TODO: actually show which classes were done -->
            <div class="lesson-outer" class:done={index < 2}>
              {index + 1}
            </div>
          </a>
        {/each}
      </div>
    </div>
  {:else}
    <p>{$t('collection.no-courses')}</p>
  {/each}

  <Footer white />
</DarkSection>

<style>
  .video-wrapper {
    width: 100%;
    margin: 2rem 0;
  }
  label {
    display: grid;
    justify-items: center;
    margin: 2rem auto;
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
    gap: 0.5rem;
  }
  .lesson-outer {
    display: grid;
    align-items: center;
    justify-items: center;
    color: var(--theme-neutral-black);
    background-color: var(--theme-main-light);
    width: 2rem;
    height: 2rem;
    word-wrap: break-word;
    border-radius: 5px;
  }
  .lesson-outer.done {
    background-color: var(--theme-main);
  }
  .course {
    display: grid;
    gap: 1rem;
    padding: 1rem;
    margin-bottom: 1rem;
    border: var(--theme-neutral-white) solid 1px;
    border-radius: 5px;
  }
</style>
