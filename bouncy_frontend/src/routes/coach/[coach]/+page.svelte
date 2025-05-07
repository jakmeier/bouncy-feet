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
  import TrackerPreview from '$lib/components/avatar/TrackerPreview.svelte';
  import { goto } from '$app/navigation';
  import { coachLocale, t } from '$lib/i18n';
  import { locale } from '$lib/i18n';

  const coachId = page.params.coach;
  const { getCourse } = getContext('courses');

  const coach = $derived(coachData(coachId));
  const courses = $derived(coach.courseIds.map(getCourse));
  const step = $derived(courses[0].featuredStep());

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
    <h2>{$t('coach.courses-title')}</h2>
    <!-- <h3>I can show you my tricks</h3> -->

    {#each courses as course}
      <p>{course.name}</p>
      <div class="ol">
        {#each course.lessons as lesson, index}
          <div class="lesson-outer">
            <div class="corner-marked2">
              <div class="corner-marked">
                <div class="lesson-inner">
                  <a href="../../courses/{course.id}/exercise/{index}">
                    <div class="preview">
                      <TrackerPreview
                        tracker={course.tracker(index)}
                        size={150}
                        backgroundColor="transparent"
                      ></TrackerPreview>
                    </div>
                    <!-- TODO: check lesson names are good -->
                    <div class="lesson-name">{lesson.name}</div>
                  </a>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/each}
    <Footer white />
  </DarkSection>
</AvatarStyleContext>

<style>
  .ol {
    display: flex;
    overflow: scroll;
    padding-bottom: 1rem;
    margin-left: -1.5rem;
    margin-right: -1.5rem;
  }
  .lesson-outer {
    padding: 0.5rem;
    max-width: min(300px, 68vw);
    font-size: var(--font-large);
    margin: 0.5rem;
    word-wrap: break-word;
  }
  .preview {
    margin: 0 0.5rem;
    height: 100%;
  }
  .lesson-name {
    text-align: center;
    padding-bottom: 1rem;
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
