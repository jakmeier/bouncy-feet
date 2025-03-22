<script>
  import { coaches } from '$lib/coach';
  import { getContext } from 'svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { coachLocale, locale, t } from '$lib/i18n';
  import { goto } from '$app/navigation';
  import TrackerPreview from '$lib/components/avatar/TrackerPreview.svelte';

  /** @type {LocalState}*/
  const localState = getContext('localState');
  const { getCourse } = getContext('courses');

  // TODO: read from user
  const selected = 'chorok';
  let coachWidth;

  function selectCoach(coach) {
    // TODO: store as the coach for lessons, rather than as user style
    Object.assign(localState.avatarStyle.headStyle, coach.style.headStyle);
    Object.assign(localState.avatarStyle.coloring, coach.style.coloring);
    Object.assign(localState.avatarStyle.bodyShape, coach.style.bodyShape);
    localState.selectedCoach = coach.name;

    goto('/');
  }
</script>

<StandardPage>
  {#each coaches as coach}
    <div
      class="coach"
      bind:clientWidth={coachWidth}
      style="height: {1.25 * coachWidth}px"
    >
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
      <div class="title">{coach.title[coachLocale($locale)]}</div>
      <div class="name" style="color: {coach.style.coloring.headColor}">
        {coach.name}
      </div>
      <div class="description">
        {coach.description[coachLocale($locale)]}
      </div>
      {#if selected === coach.name}{/if}
      <button onclick={() => selectCoach(coach)}>
        {$t('coach.select-coach-button')}
      </button>
    </div>
  {/each}
</StandardPage>

<style>
  .coach {
    display: grid;
    margin-top: 8rem;
    margin-bottom: 16rem;
    position: relative;
    z-index: 1;
  }

  .coach:last-child {
    margin-bottom: 4rem;
  }

  .coach div {
    position: absolute;
    z-index: 2;
  }

  .name {
    position: absolute;
    top: -7rem;
    text-transform: capitalize;
    font-size: calc(2 * var(--font-large));
    /* text-align: right; */
    /* mix-blend-mode: multiply; */
  }

  .title {
    top: -2.2rem;
    font-size: var(--font-normal);
  }

  .description {
    top: 7rem;
    font-size: var(--font-nnormal);
    width: 40vw;
  }

  button {
    align-self: flex-end;
    justify-self: center;
  }

  .avatar {
    position: relative;
    top: 0;
    height: min(80vw, 60dvh);
    width: min(80vw, 60dvh);
    transform: translateX(20vw);
  }
</style>
