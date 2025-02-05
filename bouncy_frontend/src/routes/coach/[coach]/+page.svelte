<script>
  import { page } from '$app/stores';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from '$lib/constants';
  import { bpm, halfSpeed } from '$lib/stores/Beat';
  import { getContext, onMount } from 'svelte';
  import { t } from '$lib/i18n';
  import { dev } from '$lib/stores/FeatureSelection';

  const coach = $page.params.coach;
  const { getCourse } = getContext('courses');
  const { setTrack } = getContext('music');

  const style = coachStyle(coach);
  const course = getCourse(coachCourseId(coach), true);
  const step = course.featuredStep();

  $bpm = 120;
  $halfSpeed = true;

  // TODO: let user turn it off
  let audioOn = true;

  function coachStyle(coach) {
    switch (coach) {
      case 'juhwang':
        return ORANGE_COLORING;
      case 'chorok':
        return LEFT_RIGHT_COLORING;
      default:
        return LEFT_RIGHT_COLORING;
    }
  }

  function coachCourseId(coach) {
    switch (coach) {
      case 'juhwang':
        return 'running-man-basics';
      case 'chorok':
        return 'v-step-basics';
      default:
        return '';
    }
  }

  onMount(() => {
    setTrack('120bpm_tech_house');
  });
</script>

<Header title={coach} />

{#if step}
  <AnimatedStep {step} size={150} {style}></AnimatedStep>
{/if}

<div class="ol">
  {#each course.lessons as lesson, index}
    <div class="course">
      <div class="step">
        {#if lesson.parts.length > 0}
          <!-- TODO: show in coach style -->
          <!-- TODO: show all parts, not just the last! -->
          <AnimatedStep
            step={lesson.parts[lesson.parts.length - 1].step}
            size={175}
          ></AnimatedStep>
        {/if}
      </div>
      <div class="link">
        <a href="../../courses/{course.id}/exercise/{index}">
          <button class="light">
            {$t('courses.course-overview.start-lesson')}
          </button>
        </a>
        {#if $dev}
          <a href="./{index}">
            <button class="light"> text explanation (WIP) </button>
          </a>
        {/if}
      </div>
    </div>
  {/each}
</div>

<!-- WIP -->
<div class="train">
  Train

  <div class="link">
    <a href="../../courses/{course.id}/exercise/-1/record">
      <button class="light">
        {$t('courses.course-overview.start-lesson')}
      </button>
    </a>
  </div>
</div>

<style>
  .ol {
    display: flex;
    overflow: scroll;
    margin: 15px 10px 10px;
    /* gap: 10px; */
    /* justify-content: center; */
  }
  .course {
    box-shadow: 0 0 4px 2px var(--theme-main);
    background-color: var(--theme-neutral-light);
    padding: 5px 10px;
    max-width: 400px;
    font-size: 30px;
    margin: 10px;
  }
  .step {
    margin: 0 15px;
  }
  .link {
    padding: 10px;
    text-align: center;
  }
  .train {
    box-shadow: 0 0 4px 2px var(--theme-main);
    background-color: var(--theme-neutral-light);
    padding: 5px 10px;
    max-width: 400px;
    font-size: 30px;
    margin: 30px auto;
    text-align: center;
  }
  button {
    margin: 10px;
    height: min-content;
    min-width: 160px;
  }
</style>
