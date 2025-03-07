<script>
  import { coaches } from '$lib/coach';
  import { getContext } from 'svelte';
  import AvatarStyleContext from '$lib/components/avatar/AvatarStyleContext.svelte';
  import AnimatedStep from '$lib/components/AnimatedStep.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import { locale } from '$lib/i18n';
  import { base } from '$app/paths';

  const { getCourse } = getContext('courses');

  // TODO: read from user
  const selected = 'chorok';
  let coachWidth;
</script>

<StandardPage white>
  {#each coaches as coach}
    <div
      class="coach"
      bind:clientWidth={coachWidth}
      style="height: {1.2 * coachWidth}px"
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
        <div class="name" style="color: {coach.style.coloring.headColor}">
          {coach.name}
        </div>
        <div class="title">{coach.title[$locale.substring(0, 2)]}</div>
        <div class="description">
          {coach.description[$locale.substring(0, 2)]}
        </div>
        {#if selected === coach.name}
          <div class="selected">
            <div>Current selection</div>
            <img src="{base}/img/symbols/bf_eye.svg" alt="bf_eye" />
          </div>
        {/if}
      </div>
    </div>
  {/each}
</StandardPage>

<style>
  .coach {
    margin-top: 5rem;
    position: relative;
    width: min(90%, 60dvh);
    z-index: 1;
  }

  .coach div {
    position: absolute;
    z-index: 2;
  }

  .name {
    top: -4rem;
    text-transform: capitalize;
    font-size: calc(2 * var(--font-large));
    text-align: right;
   */
    mix-blend-mode: multiply;
  }

  .title {
    rotate: -3deg;
    top: 0.8rem;
    font-size: var(--font-normal);
  }

  .description {
    top: 6rem;
    font-size: var(--font-nnormal);
    padding: 2rem;
    background-color: #FFFFFFFF;
  }

  .coach .selected {
    display: grid;
    grid-template-columns: min-content auto;
    gap: 1rem;
    width: 100%;
    top: -6rem;
    align-items: center;
    justify-content: end;
    z-index: 1;
  }
  .coach .selected div {
    position: relative;
  }
  .coach .selected img {
    height: 3rem;
    width: 3rem;
  }

  .avatar {
    position: relative;
    top: 0;
    height: min(80vw, 60dvh);
    width: min(80vw, 60dvh);
  }
</style>
