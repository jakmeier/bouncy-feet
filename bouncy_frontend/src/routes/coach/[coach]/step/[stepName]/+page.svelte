<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import { getContext, onMount } from 'svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm, beatCounter } from '$lib/stores/Beat';
  import { coaches } from '$lib/coach';
  import Video from '$lib/components/ui/Video.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import DanceCounts from '$lib/components/DanceCounts.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import LightSection from '$lib/components/ui/sections/LightSection.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';
  import CornerMarker from '$lib/components/ui/CornerMarker.svelte';
  import NumberSlider from '$lib/components/ui/NumberSlider.svelte';

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
