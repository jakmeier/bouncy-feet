<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import { getContext, onMount } from 'svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm, beatCounter } from '$lib/stores/Beat';
  import { coaches } from '$lib/coach';
  import Video from '$lib/components/ui/Video.svelte';
  import DanceCounts from '$lib/components/DanceCounts.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import CornerMarker from '$lib/components/ui/CornerMarker.svelte';
  import NumberSlider from '$lib/components/ui/NumberSlider.svelte';
  import ContextStyledSection from '$lib/components/ui/sections/ContextStyledSection.svelte';

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
    document.querySelector('.background')?.scrollTo(0, 0);
    $bpm = 45;
  });
</script>

<NightSection fillScreen arrow>
  <LogoHeader title={name} backButton />
  <!-- TODO: could be nice to have square videos here for better screen fitting -->
  <div class="video-wrapper">
    {#if video && video.length > 0}
      <Video path={`${video}`}></Video>
    {/if}
  </div>
</NightSection>

<LightSection fillScreen arrow arrowText={$t('collection.step.show-courses')}>
  <div class="counts">
    <DanceCounts steps={[step]} markedPoseIndex={$beatCounter} />
  </div>

  <div class="animation-outer">
    <CornerMarker color="var(--theme-neutral-white)">
      <div class="animation-inner">
        <AnimatedStep {step} size={200} backgroundColor="transparent"
        ></AnimatedStep>
        <div class="slider">
          <NumberSlider
            name="bpm-slider"
            bind:value={$bpm}
            min={15}
            max={200}
            decimals={0}
            thumbColor={coach.style.pageColoring.pageColor}
            backgroundColor="var(--theme-neutral-black)"
            unitLabel="BPM"
          ></NumberSlider>
        </div>
      </div>
    </CornerMarker>
  </div>
</LightSection>

<ContextStyledSection pageColoring={coach.style.pageColoring}>
  <h2>{$t('collection.courses-subtitle')}</h2>

  {#each courses as course}
    <div class="course">
      <div>
        <div class="course-name">{name}</div>
        <h2>{course.name}</h2>
      </div>

      <a href="../../../../courses/{course.id}">
        <button>{$t('courses.course-overview.start-lesson')}</button>
      </a>
      <div class="lessons-subtitle">
        {$t('courses.course-overview.lessons-subtitle')}
      </div>
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
</ContextStyledSection>

<style>
  .video-wrapper {
    width: calc(100% - 1rem);
    margin-bottom: 2rem;
  }

  .counts {
    margin-top: 2rem;
  }

  .animation-outer {
    margin: 1rem auto;
  }

  .slider {
    padding-bottom: 1rem;
  }

  .ol {
    grid-column-start: 1;
    grid-column-end: 3;
    display: flex;
    overflow: scroll;
    padding-bottom: 1rem;
    gap: 1rem;
    flex-wrap: wrap;
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
    grid-template-columns: 2fr 110px;
    /* padding: 1rem; */
    margin-bottom: 1rem;
    /* background-color: var(--theme-accent-medium); */
    /* color: var(--theme-neutral-white); */
    /* border-radius: 1rem; */
  }
  .course h2 {
    margin: 0;
  }
  .lessons-subtitle {
    margin-top: 1rem;
  }
  .course > a {
    align-self: center;
    justify-self: end;
    width: 100%;
  }
  .course a button {
    display: block;
    margin-left: auto;
    border: 3px solid var(--theme-accent-medium);
  }
</style>
