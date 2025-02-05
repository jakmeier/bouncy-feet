<script>
  import { page } from '$app/stores';
  import { t } from '$lib/i18n.js';
  import Header from '$lib/components/ui/Header.svelte';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import { counter } from '$lib/timer';
  import { dev } from '$lib/stores/FeatureSelection.js';
  import Symbol from '$lib/components/ui/Symbol.svelte';

  const { getCourse } = getContext('courses');
  const user = getContext('user').store;

  const id = $page.params.courseId;
  const course = getCourse(id);

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);

  $: courseProgress = $user.userLessonProgress[id];
</script>

<Header title={course.name} />

<p>{course.explanation}</p>

<div class="ol">
  <!-- WIP -->
  <div class="course">
    Train

    <div class="link">
      <a href="./exercise/-1/record">
        <button class="light">
          {$t('courses.course-overview.start-lesson')}
        </button>
      </a>
    </div>
  </div>

  {#each course.lessons as lesson, index}
    <div class="course">
      <div>
        {$t('courses.lesson.title')}
        {index + 1}
      </div>
      <div class="step">
        {#if lesson.parts.length > 0}
          <Step
            step={lesson.parts[lesson.parts.length - 1].step}
            poseIndex={$i}
            {animationTime}
            size={175}
            borderWidth={0}
            lineWidth={8}
          ></Step>
        {/if}
        <div class="rank">
          {#if courseProgress && courseProgress.lessons}
            {#if courseProgress.lessons[index] > 0}
              <Symbol size={50} class="blue">verified</Symbol>
            {/if}

            {#each Array(Math.max(0, courseProgress.lessons[index] - 1 || 0)) as _}
              <Symbol size={50}>star</Symbol>
            {/each}
          {/if}
        </div>
      </div>
      <div class="link">
        <a href="./exercise/{index}">
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

<style>
  p {
    text-align: center;
    box-shadow: 0 0 4px 2px var(--theme-main);
    padding: 15px;
    border-radius: 20px;
    background-color: var(--theme-neutral-light);
  }
  .ol {
    display: grid;
    margin: 15px 10px 10px;
    gap: 10px;
    justify-content: center;
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
  .rank {
    display: flex;
    justify-content: center;
    margin-top: -5px;
  }
  button {
    margin: 10px;
  }
</style>
