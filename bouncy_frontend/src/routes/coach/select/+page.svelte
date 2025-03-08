<script>
  import { coaches } from '$lib/coach';
  import { getContext } from 'svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { locale, t } from '$lib/i18n';
  import { base } from '$app/paths';

  const { getCourse } = getContext('courses');

  // TODO: read from user
  const selected = 'chorok';
  let coachWidth;

  function selectCoach(coach) {
    console.log('selected', coach);
  }
</script>

<StandardPage white>
  <div class="coaches">
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
            <AnimatedStep
              step={getCourse(coach.courseIds[0]).lessons[0].parts[0].step}
              size={coachWidth}
              backgroundColor={'var(--theme-neutral-light)'}
            ></AnimatedStep>
          </AvatarStyleContext>
        </div>
        <div class="title">{coach.title[$locale.substring(0, 2)]}</div>
        <div class="name" style="color: {coach.style.coloring.headColor}">
          {coach.name}
        </div>
        <div class="description">
          {coach.description[$locale.substring(0, 2)]}
        </div>
        {#if selected === coach.name}
          <!-- <div class="selected">
      <div>Current selection</div>
      <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
      </div> -->
        {/if}
        <button onclick={() => selectCoach(coach)}>
          {$t('coach.select-coach-button')}
        </button>
      </div>
    {/each}
  </div>
</StandardPage>

<style>
  .coaches {
    /* padding: 10rem 0; */
  }
  .coach {
    display: grid;
    margin-top: 8rem;
    margin-bottom: 16rem;
    position: relative;
    width: min(90%, 60dvh);
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
    rotate: -3deg;
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
    justify-self: end;
  }

  .avatar {
    position: relative;
    top: 0;
    height: min(80vw, 60dvh);
    width: min(80vw, 60dvh);
    transform: translateX(20vw);
  }
</style>
