<script>
  // Persistent shared-element overlay for route transitions. It is rendered by
  // the root layout, so its clones survive the route swap.
  import { pageTransition as t } from '$lib/stores/UiState.svelte';

  let surfaceEl = $state();
  let titleEl = $state();
  const surfaceBase = $derived(
    t.surface
      ? `top:${t.surface.top}px;left:${t.surface.left}px;width:${t.surface.width}px;height:${t.surface.height}px;border-radius:${t.surface.borderRadius}px;`
      : null
  );
  const titleBase = $derived(
    t.title
      ? `top:${t.title.top}px;left:${t.title.left}px;width:${t.title.width}px;font-size:${t.title.fontSize}px;color:${t.destTitle ? t.destTitle.color : t.title.color};font-weight:${t.destTitle ? t.destTitle.fontWeight : t.title.fontWeight};`
      : null
  );

  // Expand the final selected-card surface while retaining the source title.
  $effect(() => {
    if (
      t.kind !== 'challenge' ||
      t.phase !== 'expand' ||
      !t.surface ||
      !surfaceEl
    )
      return;
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
  });

  // Square off the full-card border while the finished challenge overlay fades.
  $effect(() => {
    if (t.kind !== 'challenge' || t.phase !== 'finishing' || !surfaceEl) return;
    surfaceEl.animate(
      [
        { borderRadius: getComputedStyle(surfaceEl).borderRadius },
        { borderRadius: '0px' },
      ],
      { duration: 400, easing: 'ease-out', fill: 'forwards' }
    );
  });

  // Challenge phase 2: settle clones onto the elements published by the
  // destination page.
  $effect(() => {
    if (t.kind !== 'challenge' || !t.destTitle || !t.title || !titleEl) return;
    const s = t.title;
    const d = t.destTitle;
    titleEl.animate(
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

  // Coach phase: preserve the selected personality title while its source
  // page fades, then enlarge it before navigation.
  $effect(() => {
    if (
      t.kind !== 'coach' ||
      t.phase !== 'enlarge-title' ||
      !t.title ||
      !titleEl
    )
      return;
    const s = t.title;
    titleEl.animate(
      [
        {
          top: `${s.top}px`,
          left: `${s.left}px`,
          width: `${s.width}px`,
          fontSize: `${s.fontSize}px`,
        },
        {
          top: `${s.top}px`,
          left: `${s.left}px`,
          width: `${s.width}px`,
          fontSize: '36px',
        },
      ],
      { duration: 600, easing: 'linear', fill: 'forwards' }
    );
  });

  // The destination header publishes its real rect after navigation. This is
  // the second half of the coach transition, replacing the old title-only
  // crossfade with the same explicit shared-element settle used by challenges.
  $effect(() => {
    if (
      t.kind !== 'coach' ||
      t.phase !== 'settle' ||
      !t.title ||
      !t.destTitle ||
      !titleEl
    )
      return;
    const s = t.title;
    const d = t.destTitle;
    titleEl.animate(
      [
        {
          top: `${s.top}px`,
          left: `${s.left}px`,
          width: `${s.width}px`,
          fontSize: '36px',
          color: s.color,
          fontWeight: '700',
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
      { duration: 600, easing: 'ease', fill: 'forwards' }
    );
  });
</script>

{#if t.active}
  <div
    class="pt-overlay"
    class:challenge={t.kind === 'challenge'}
    class:coach={t.kind === 'coach'}
    class:finishing={t.finishing}
    aria-hidden="true"
  >
    {#if surfaceBase}<div
        class="pt-surface"
        bind:this={surfaceEl}
        style={surfaceBase}
      ></div>{/if}
    {#if titleBase}<div class="pt-title" bind:this={titleEl} style={titleBase}>
        {t.title?.text}
      </div>{/if}
  </div>
{/if}

<style>
  .pt-overlay {
    position: fixed;
    inset: 0;
    z-index: 2000;
    pointer-events: none;
    transition: opacity 360ms ease;
  }
  .pt-overlay.finishing {
    opacity: 0;
  }
  .pt-overlay.coach.finishing {
    transition-duration: 600ms;
  }
  .pt-surface {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1;
    box-sizing: border-box;
    border: 2px solid var(--theme-neutral-white);
    background: var(--theme-neutral-black);
  }
  .pt-title {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 2;
    margin: 0;
    line-height: 1.1;
    letter-spacing: 0.02em;
    white-space: pre-wrap;
  }
  /* The legacy coach surrogate was an h1; retain its line metrics so the
     source title does not jump vertically before its enlargement. */
  .pt-overlay.coach .pt-title {
    line-height: normal;
  }
</style>
