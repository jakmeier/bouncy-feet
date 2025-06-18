<script>
  import { coaches } from '$lib/coach';
  import { getContext } from 'svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { coachLocale, locale, t } from '$lib/i18n';
  import TrackerPreview from '$lib/components/avatar/TrackerPreview.svelte';
  import FormattedText from '$lib/components/ui/FormattedText.svelte';

  /** @type {LocalState}*/
  const { getCourse } = getContext('courses');

  let coachWidth;
</script>

{#each coaches as coach}
  <div class="coach" bind:clientWidth={coachWidth}>
    <div class="avatar">
      <AvatarStyleContext
        coloring={coach.style.coloring}
        bodyShape={coach.style.bodyShape}
        headStyle={coach.style.headStyle}
      >
        <TrackerPreview
          tracker={getCourse(coach.courseIds[0]).tracker(0)}
          size={coachWidth}
          backgroundColor={'var(--theme-neutral-black)'}
        ></TrackerPreview>
      </AvatarStyleContext>
    </div>
    <div class="text" style="min-height: {coachWidth}px">
      <div class="title" style="color: {coach.style.coloring.headColor}">
        {coach.title[coachLocale($locale)]}
      </div>
      <!-- <div class="name">
            with {coach.name}
            </div> -->
      <div class="description">
        <FormattedText
          text={coach.description[coachLocale($locale)]}
          color={coach.style.coloring.headColor}
        ></FormattedText>
      </div>
    </div>
    <div class="buttons">
      <!-- <a href="" class="button"> Basic Moves </a>
                <a href="" class="button"> Video Clips </a>
                <a href="" class="button"> Courses </a> -->
      <a href="./coach/{coach.name}">
        <button> Basic Moves </button>
      </a>
      <button> Video Clips </button>
      <button> Courses </button>
    </div>
  </div>
{/each}

<style>
  .coach {
    display: grid;
    grid-template-columns: 2fr 1fr;
    margin-top: 4rem;
    margin-bottom: 0rem;
    position: relative;
  }

  .avatar {
    position: absolute;
    top: 0;
    height: min(80vw, 60dvh);
    width: min(80vw, 60dvh);
    transform: translateX(30vw);
  }

  .text,
  .buttons {
    position: relative;
    z-index: 2;
  }

  .title {
    font-size: calc(2 * var(--font-large));
  }

  .description {
    font-size: var(--font-normal);
    width: 40vw;
  }

  .buttons {
    grid-column-start: 1;
    grid-column-end: 3;
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
</style>
