<script>
  import { page } from '$app/stores';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from '$lib/constants';
  import { bpm, halfSpeed } from '$lib/stores/Beat';
  import { getContext, onDestroy, onMount } from 'svelte';
  import { t } from '$lib/i18n';
  import BackButton from '$lib/components/ui/BackButton.svelte';
  import { backgroundColor } from '$lib/stores/UiState';

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

  let swapBackgroundColor = 'var(--theme-neutral-white)';

  onMount(() => {
    setTrack('120bpm_tech_house');
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-neutral-light)';
  });
  onDestroy(() => {
    $backgroundColor = swapBackgroundColor;
  });
</script>

<!-- <Header title={coach} /> -->
<BackButton />

<!-- TODO: translated texts -->
<h2>Chorok: Master of heel-toe movements</h2>
<h3>Ready for a training session with me?</h3>
{#if step}
  <AnimatedStep {step} size={150} {style} backgroundColor="transparent"
  ></AnimatedStep>
{/if}

<div class="train">
  <div class="link">
    <a href="../../courses/{course.id}/exercise/-1/record">
      <button class="light big">
        <!-- {$t('courses.course-overview.start-lesson')} -->
        Train
      </button>
    </a>
  </div>
</div>

<!-- TODO: translated texts -->
<h2>Learn something new</h2>
<h3>I can show you my tricks</h3>

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
            backgroundColor="transparent"
          ></AnimatedStep>
        {/if}
      </div>
      <!-- TODO: lesson names -->
      <div class="lesson-name">Lesson X</div>
      <div class="link">
        <a href="../../courses/{course.id}/exercise/{index}">
          <button class="light">
            {$t('courses.course-overview.start-lesson')}
          </button>
        </a>
      </div>
    </div>
  {/each}
</div>

<style>
  h2,
  h3,
  .ol {
    color: black;
  }
  .ol {
    display: flex;
    overflow: scroll;
    margin: 15px 10px 10px;
    /* gap: 10px; */
    /* justify-content: center; */
  }
  .course {
    /* box-shadow: 0 0 4px 2px var(--theme-main); */
    /* background-color: var(--theme-neutral-light); */
    padding: 5px 10px;
    max-width: 400px;
    font-size: var(--font-large);
    margin: 10px;
  }
  .step {
    margin: 0 15px;
  }
  .link {
    padding: 10px;
    text-align: center;
  }
  .lesson-name {
    text-align: center;
  }

  button {
    background-color: var(--theme-accent-light);
    margin: 10px;
    height: min-content;
    min-width: 160px;
  }

  .big {
    background-color: var(--theme-main);
    font-size: var(--font-large);
    min-width: 250px;
  }
</style>
