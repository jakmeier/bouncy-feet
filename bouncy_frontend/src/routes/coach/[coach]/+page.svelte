<script>
  import { page } from '$app/state';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm } from '$lib/stores/Beat';
  import { getContext } from 'svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import { coaches } from '$lib/coach';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import { goto } from '$app/navigation';
  import { coachLocale, t } from '$lib/i18n';
  import { locale } from '$lib/i18n';
  import { stepById, StepWrapper } from '$lib/instructor/bouncy_instructor';
  import ContextStyledSection from '$lib/components/ui/sections/ContextStyledSection.svelte';
  import VideoFeed from '$lib/components/VideoFeed.svelte';

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
    Object.keys(coach.steps)
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

<AvatarStyleContext
  coloring={coach.style.coloring}
  bodyShape={coach.style.bodyShape}
  headStyle={coach.style.headStyle}
>
  <ContextStyledSection
    pageColoring={coach.style.pageColoring}
    fillScreen
    arrow
    arrowText={$t('collection.to-steps')}
  >
    <LogoHeader
      transparent
      backButton
      {onBack}
      title={coach.title[coachLocale($locale)]}
    />
    <!-- TODO(July): add "upload" button top right -->
    <VideoFeed />
  </ContextStyledSection>

  <ContextStyledSection pageColoring={coach.style.pageColoring}>
    <h2>{$t('collection.steps-subtitle')}</h2>

    {#each steps as step}
      <a href={`./step/${step.name}`}>
        <div
          class="step"
          style="border-color: {coach.style.pageColoring.secondaryColor};"
        >
          <AnimatedStep {step} size={100} backgroundColor="var(--dance-floor)"
          ></AnimatedStep>
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

    <Footer />
  </ContextStyledSection>
</AvatarStyleContext>

<style>
  .step {
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    background-color: var(--dance-floor);
    margin: 1rem 0;
    border-radius: 1rem;
    border: 3px solid;
  }

  .step h3 {
    margin: 0;
  }
</style>
