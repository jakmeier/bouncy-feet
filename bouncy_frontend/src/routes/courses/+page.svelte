<script>
  import { t } from '$lib/i18n';
  import { getContext, onDestroy, onMount } from 'svelte';
  import Header from '$lib/components/ui/Header.svelte';
  import { backgroundColor } from '$lib/stores/UiState';
  import { WHITE_COLORING } from '$lib/constants';
  import Step from '../collection/Step.svelte';
  import { counter } from '$lib/timer';
  import { timeBetweenMoves } from '$lib/stores/Beat';

  const { courses } = getContext('courses');
  const beat = counter(-1, 1, $timeBetweenMoves);

  let swapBackgroundColor = 'var(--theme-neutral-white)';
  onMount(() => {
    swapBackgroundColor = $backgroundColor;
    $backgroundColor = 'var(--theme-main)';
  });
  onDestroy(() => {
    $backgroundColor = swapBackgroundColor;
  });
</script>

<Header title={$t('courses.title')} />

{#each $courses as course}
  <a class="course" href={course.id}>
    <div class="course-rect">
      <Step
        step={course.featuredStep()}
        poseIndex={$beat}
        animationTime={$timeBetweenMoves * 0.6}
        style={WHITE_COLORING}
        borderWidth={0}
      />
      <div class="start-here">start here</div>
    </div>
    <div class="course-name">{course.name}</div>
  </a>
{/each}

<div class="grayed-out">
  <div class="course-rect">
    <div class="course-symbol">?</div>
    <div class="course-symbol">?</div>
  </div>
  <div class="course-name">{$t('courses.more-coming')}</div>
</div>

<style>
  .course {
    text-decoration: none;
    color: var(--theme-neutral-white);
  }
  .course-rect {
    display: grid;
    grid-template-columns: 1fr 1fr;
    justify-items: center;
    align-items: center;
    background-color: var(--theme-accent);
    width: 90%;
    margin: 25px auto 5px;
    padding: 15px;
    box-shadow: var(--theme-neutral-white) 0px 0px 5px;
    line-height: 72px;
  }
  .course-symbol {
    text-align: center;
    padding: 20px;
    font-size: 48px;
  }
  .course-name {
    text-align: center;
    font-weight: 800;
    font-size: 1.25em;
    color: var(--theme-neutral-white);
  }
  .grayed-out {
    color: var(--theme-neutral-dark);
  }
  .grayed-out .course-rect {
    background-color: var(--theme-neutral-gray);
    box-shadow: var(--theme-neutral-dark) 0px 0px 5px;
  }
  .start-here {
    background-color: var(--theme-accent-light);
    color: var(--theme-neutral-white);
    text-shadow: var(--theme-neutral-dark) 0px 0px 11px;
    font-size: 28px;
    line-height: 28px;
    text-align: center;
    padding: 20px;
    border-radius: 10px;
    justify-self: center;
    align-self: center;
  }

  @media (max-width: 360px) {
    .course-rect {
      grid-template-columns: 1fr min-content;
    }
  }
</style>
