<script>
  import { page } from '$app/state';
  import { getContext, onMount } from 'svelte';
  import { counter } from '$lib/timer';
  import Background from '$lib/components/ui/sections/Background.svelte';
  import Video from '$lib/components/ui/Video.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import NightSection from '$lib/components/ui/sections/NightSection.svelte';

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

  onMount(() => {
    document.querySelector('.background')?.scrollTo(0, 0);
  });
</script>

<NightSection arrow fillScreen>
  <LogoHeader title={course.name} backButton />

  <div>{course.explanation}</div>

  <div class="video-wrapper">
    {#if video && video.length > 0}
      <Video path={`${video}`}></Video>
    {/if}
  </div>
</NightSection>

<Background bgColor="var(--theme-main-alt)" color="var(--theme-neutral-black)"
></Background>

<div class="ol">
  {#each course.lessons as lesson, index}
    <a href="./exercise/{index}" class="lesson">
      <div class="lesson-number">
        {index + 1}
      </div>
      <div class="lesson-name">
        {lesson.name}
      </div>
    </a>
  {/each}
</div>

<Footer></Footer>

<style>
  .ol {
    display: grid;
    margin: 15px 10px 10px;
    gap: 10px;
    justify-content: center;
  }
  .lesson {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    font-size: var(--font-large);
    margin-top: 2rem;
  }
  .video-wrapper {
    width: calc(100% - 1rem);
    margin-bottom: 2rem;
  }

  .lesson-number {
    background-color: var(--theme-main);
    border-radius: 1rem;
    width: min(40vw, 200px);
    height: min(40vw, 200px);
    font-size: min(30vw, 150px);
    justify-items: center;
    aspect-ratio: 1 / 1;
  }

  .lesson-number,
  .lesson-name {
    display: grid;
    align-items: center;
  }

  .lesson:nth-of-type(even) {
    flex-direction: row-reverse;
  }
</style>
