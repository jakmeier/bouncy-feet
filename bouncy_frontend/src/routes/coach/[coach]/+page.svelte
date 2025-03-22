<script>
  import { page } from '$app/state';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import { bpm, halfSpeed } from '$lib/stores/Beat';
  import { getContext } from 'svelte';
  import LightBackground from '$lib/components/ui/sections/LightBackground.svelte';
  import DarkSection from '$lib/components/ui/sections/DarkSection.svelte';
  import Footer from '$lib/components/ui/Footer.svelte';
  import LogoHeader from '$lib/components/ui/LogoHeader.svelte';
  import { coaches } from '$lib/coach';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import TrackerPreview from '$lib/components/avatar/TrackerPreview.svelte';
  import { goto } from '$app/navigation';

  const coachId = page.params.coach;
  const { getCourse } = getContext('courses');

  const coach = $derived(coachData(coachId));
  const course = $derived(getCourse(coach.courseIds[0]));
  const step = $derived(course.featuredStep());

  $bpm = 120;
  $halfSpeed = true;

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
</script>

<LightBackground />

<LogoHeader title={coachId} backButton white {onBack} />

<AvatarStyleContext
  coloring={coach.style.coloring}
  bodyShape={coach.style.bodyShape}
  headStyle={coach.style.headStyle}
>
  <!-- TODO: translated texts -->
  <h3>Master of heel-toe movements</h3>
  <!-- <h3>Ready for a training session with me?</h3> -->
  {#if step}
    <AnimatedStep {step} size={150} backgroundColor="transparent"
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

  <!-- TODO: translated texts -->
  <DarkSection>
    <h2>Learn something new</h2>
    <h3>I can show you my tricks</h3>

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
                      size={175}
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
    max-width: 400px;
    font-size: var(--font-large);
    margin: 0.5rem;
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
