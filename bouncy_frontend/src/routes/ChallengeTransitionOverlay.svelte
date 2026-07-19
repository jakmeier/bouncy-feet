<script>
  // Persistent overlay that animates a challenge-set card into the challenge
  // page: the black surface expands to fill the page while the title and
  // progress pill glide from their card positions into the destination page's
  // header title and hero progress. Driven entirely by `challengeTransition`
  // so it survives the route swap (rendered by the root layout).
  //
  // We use the Web Animations API (element.animate) rather than CSS transitions
  // so the motion always runs from an explicit source keyframe, even though the
  // elements are inserted into the DOM at the same moment the animation starts.
  import { challengeTransition as t } from '$lib/stores/UiState.svelte';

  /** @type {HTMLDivElement | undefined} */
  let surfaceEl = $state();
  /** @type {HTMLDivElement | undefined} */
  let titleEl = $state();
  /** @type {HTMLDivElement | undefined} */
  let progressEl = $state();

  // Base (pre/post-animation) inline styles position each clone at its source
  // rect. The WAAPI animations override these while running and hold the final
  // state via `fill: 'forwards'`.
  const surfaceBase = $derived(
    t.surface
      ? `top:${t.surface.top}px;left:${t.surface.left}px;width:${t.surface.width}px;height:${t.surface.height}px;border-radius:${t.surface.borderRadius}px;`
      : null
  );

  // The title's font-weight isn't smoothly animatable, so switch it to the
  // destination weight as soon as we know it (i.e. at the start of the settle
  // phase) so there is no pop when the overlay is removed.
  const titleBase = $derived(
    t.title
      ? `top:${t.title.top}px;left:${t.title.left}px;width:${t.title.width}px;font-size:${t.title.fontSize}px;color:${t.destTitle ? t.destTitle.color : t.title.color};font-weight:${t.destTitle ? t.destTitle.fontWeight : t.title.fontWeight};`
      : null
  );

  const progressBase = $derived(
    t.progress
      ? `top:${t.progress.top}px;left:${t.progress.left}px;width:${t.progress.width}px;`
      : null
  );

  // Phase 1: expand the black surface from the card to the full viewport.
  $effect(() => {
    if (!t.active || !t.surface || !surfaceEl) return;
    const s = t.surface;
    surfaceEl.animate(
      [
        {
          top: `${s.top}px`,
          left: `${s.left}px`,
          width: `${s.width}px`,
          height: `${s.height}px`,
          borderRadius: `${s.borderRadius}px`,
        },
        {
          top: '0px',
          left: '0px',
          width: `${window.innerWidth}px`,
          height: `${window.innerHeight}px`,
          borderRadius: `${s.borderRadius}px`,
        },
      ],
      { duration: 400, easing: 'ease-in', fill: 'forwards' }
    );
    if (t.title && titleEl) {
      const kf = {
        top: `${t.title.top}px`,
        left: `${t.title.left}px`,
        width: `${t.title.width}px`,
        fontSize: `${t.title.fontSize}px`,
        color: t.title.color,
        fontWeight: t.title.fontWeight,
      };
      titleEl.animate([kf, kf], { duration: 400, fill: 'forwards' });
    }
    if (t.progress && progressEl) {
      const kf = {
        top: `${t.progress.top}px`,
        left: `${t.progress.left}px`,
        width: `${t.progress.width}px`,
      };
      progressEl.animate([kf, kf], { duration: 400, fill: 'forwards' });
    }
  });

  // Phase 2: glide the title into the destination header position once the
  // target page has published its rect.
  $effect(() => {
    if (!t.destTitle || !t.title || !titleEl) return;
    const s = t.title;
    const d = t.destTitle;
    const anim = titleEl.animate(
      [
        {
          top: `${s.top}px`,
          left: `${s.left}px`,
          width: `${s.width}px`,
          fontSize: `${s.fontSize}px`,
          color: s.color,
          fontWeight: s.fontWeight,
        },
        {
          top: `${d.top}px`,
          left: `${d.left}px`,
          width: `${d.width}px`,
          fontSize: `${d.fontSize}px`,
          color: d.color,
          fontWeight: d.fontWeight,
        },
      ],
      { duration: 440, easing: 'ease', fill: 'forwards' }
    );
  });

  // Phase 2: glide the progress pill into the destination hero position.
  $effect(() => {
    if (!t.destProgress || !t.progress || !progressEl) return;
    const s = t.progress;
    const d = t.destProgress;
    const anim = progressEl.animate(
      [
        { top: `${s.top}px`, left: `${s.left}px`, width: `${s.width}px` },
        { top: `${d.top}px`, left: `${d.left}px`, width: `${d.width}px` },
      ],
      { duration: 440, easing: 'ease', fill: 'forwards' }
    );
  });
</script>

{#if t.active}
  <div class="ct-overlay" class:finishing={t.finishing} aria-hidden="true">
    <div class="ct-surface" bind:this={surfaceEl} style={surfaceBase}></div>
    {#if titleBase}
      <div class="ct-title" bind:this={titleEl} style={titleBase}>
        {t.title?.text}
      </div>
    {/if}
    {#if progressBase}
      <div class="ct-progress" bind:this={progressEl} style={progressBase}>
        {@html t.progress?.html}
      </div>
    {/if}
  </div>
{/if}

<style>
  .ct-overlay {
    position: fixed;
    inset: 0;
    z-index: 2000;
    pointer-events: none;
    transition: opacity 360ms ease;
  }
  .ct-overlay.finishing {
    opacity: 0;
  }

  .ct-surface {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1;
    background: var(--theme-neutral-black);
  }

  .ct-title {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 2;
    margin: 0;
    line-height: 1.1;
    letter-spacing: 0.02em;
    white-space: pre-wrap;
  }

  .ct-progress {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 3;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.3rem;
    padding: 0.25rem 0.55rem;
    border-radius: 999px;
    background: var(--theme-neutral-light);
    color: var(--theme-neutral-almost-black);
    font-weight: 700;
    white-space: nowrap;
  }

  .ct-progress :global(img) {
    width: 1rem;
    height: 1rem;
  }
</style>
