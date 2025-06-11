<script>
  import { page } from '$app/state';
  import { t } from '$lib/i18n.js';
  import { getContext } from 'svelte';
  import Step from '../../collection/Step.svelte';
  import { counter } from '$lib/timer';
  import { dev } from '$lib/stores/FeatureSelection.js';
  import Symbol from '$lib/components/ui/Symbol.svelte';
  import Background from '$lib/components/ui/sections/Background.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';

  const { getCourse } = getContext('courses');
  const user = getContext('user').store;

  const id = page.params.courseId;
  /** @type {import('$lib/instructor/bouncy_instructor').Course} */
  const course = getCourse(id);
  const video = $derived(course.video);

  const stepTime = 300;
  const animationTime = stepTime * 0.7;
  const i = counter(-1, 1, stepTime);

  let courseProgress = $derived($user.userLessonProgress[id]);
</script>

<Background bgColor="var(--theme-main-alt)" color="var(--theme-neutral-black)"
></Background>

<LogoHeader title={course.name} backButton mainColor />

<p>{course.explanation}</p>

<div class="video-wrapper">
  {#if video && video.length > 0}
    <Video path={`${video}`}></Video>
  {/if}
</div>

<div class="ol">
  <!-- WIP
  <div class="course">
    Train

    <div class="link">
      <a href="./exercise/-1/record">
        <button>
          {$t('courses.course-overview.start-lesson')}
        </button>
      </a>
    </div>
  </div> -->

  {#each course.lessons as lesson, index}
    <div class="course">
      <div>
        {$t('courses.lesson.title')}
        {index + 1}
      </div>
      <div>
        {lesson.name}
      </div>
      <div class="step">
        {#if lesson.parts.length > 0}
          <Step
            step={lesson.parts[lesson.parts.length - 1].step}
            poseIndex={$i}
            {animationTime}
            size={175}
            borderWidth={0}
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
          <button>
            {$t('courses.course-overview.start-lesson')}
          </button>
        </a>
        {#if $dev}
          <a href="./{index}">
            <button> text explanation (WIP) </button>
          </a>
        {/if}
      </div>
    </div>
  {/each}
</div>

<Footer></Footer>

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
    font-size: var(--font-large);
    margin: 10px;
    border-radius: 5px;
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
  .video-wrapper {
    width: 100%;
    margin: 2rem 0;
  }
</style>
