<script>
  import { page } from '$app/state';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { LEFT_RIGHT_COLORING, ORANGE_COLORING } from '$lib/constants';
  import { bpm, halfSpeed } from '$lib/stores/Beat';
  import { getContext } from 'svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import Header from '$lib/components/ui/Header.svelte';

  const coach = page.params.coach;
  const { getCourse } = getContext('courses');

  const style = coachStyle(coach);
  const course = getCourse(coachCourseId(coach), true);
  const step = course.featuredStep();

  $bpm = 120;
  $halfSpeed = true;

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
</script>

<LightBackground />

<Header title={coach} />

<!-- TODO: translated texts -->
<h3>Master of heel-toe movements</h3>
<!-- <h3>Ready for a training session with me?</h3> -->
{#if step}
  <AnimatedStep {step} size={150} backgroundColor="transparent"></AnimatedStep>
{/if}

<div class="train">
  <div class="link">
    <a href="../../courses/{course.id}/exercise/-1/record">
      <button>
        <!-- {$t('courses.course-overview.start-lesson')} -->
        Train
      </button>
    </a>
  </div>
</div>

<!-- TODO: translated texts -->
<DarkSection>
  <h2>Learn something new</h2>
  <h3>I can show you my tricks</h3>

  <div class="ol">
    {#each course.lessons as lesson, index}
      <div class="corner-marked2">
        <div class="course corner-marked">
          <a href="../../courses/{course.id}/exercise/{index}">
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
          </a>
        </div>
      </div>
    {/each}
  </div>
  <Footer white />
</DarkSection>

<style>
  .ol {
    display: flex;
    overflow: scroll;
    padding-bottom: 1rem;
    margin-left: -1rem;
  }
  .course {
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
    margin: 10px;
    height: min-content;
    min-width: 160px;
  }
  .train {
    margin-bottom: 3rem;
  }
</style>
