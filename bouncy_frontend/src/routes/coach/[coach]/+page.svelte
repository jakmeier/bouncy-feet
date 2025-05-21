<script>
  import { page } from '$app/state';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm } from '$lib/stores/Beat';
  import { getContext } from 'svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import { coaches } from '$lib/coach';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import { goto } from '$app/navigation';
  import { coachLocale, t } from '$lib/i18n';
  import { locale } from '$lib/i18n';
  import { stepById, StepWrapper } from '$lib/instructor/bouncy_instructor';

  /**
   * @typedef {Object} Props
   * @property {import('./$types').PageData} data
   */

  /** @type {Props} */
  let { data } = $props();

  const coachId = page.params.coach;
  const { getCourse } = getContext('courses');

  const coach = $derived(coachData(coachId));
  const courses = $derived(coach.courseIds.map(getCourse));
  const step = $derived(courses[0].featuredStep());

  /** @type {StepWrapper[]} */
  const steps = $derived(
    coach.stepIds
      .map((stepId) => stepById(stepId, false))
      .filter((maybe) => maybe)
  );

  $bpm = 120;

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

  function onBack() {
    goto('/', { replaceState: true });
  }

  const title = coachId.charAt(0).toUpperCase() + coachId.slice(1);
</script>

<LightBackground />

<LogoHeader {title} backButton white {onBack} />

<AvatarStyleContext
  coloring={coach.style.coloring}
  bodyShape={coach.style.bodyShape}
  headStyle={coach.style.headStyle}
>
  <h3>{coach.title[coachLocale($locale)]}</h3>
  {#if step}
    <AnimatedStep {step} size={350} backgroundColor="transparent"
    ></AnimatedStep>
  {/if}

  <!-- 
<div class="train">
  <div class="link">
    <a href="../../courses/{course.id}/exercise/-1/record">
      <button>
        {$t('courses.course-overview.start-lesson')}
        Train
      </button>
    </a>
  </div>
</div> -->

  <DarkSection>
    <!-- <h2>{$t('coach.courses-title')}</h2> -->
    <!-- <h3>I can show you my tricks</h3> -->
    <h2>{$t('collection.steps-subtitle')}</h2>

    {#each steps as step}
      <a href={`./step/${step.name}`}>
        <div class="step">
          <AnimatedStep {step} size={100} style="round"></AnimatedStep>
          <!-- TODO: translations -->
          <h3>{step.name}</h3>
        </div>
      </a>
    {:else}
      <p>{$t('collection.no-steps')}</p>
    {/each}

    <h2>{$t('collection.combos-subtitle')}</h2>
    <p>{$t('collection.no-combos')}</p>

    <h2>{$t('collection.dances-subtitle')}</h2>
    <p>{$t('collection.no-choreos')}</p>

    <Footer white />
  </DarkSection>
</AvatarStyleContext>

<style>
  .step {
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
  }

  .step h3 {
    margin: 0;
  }
</style>
