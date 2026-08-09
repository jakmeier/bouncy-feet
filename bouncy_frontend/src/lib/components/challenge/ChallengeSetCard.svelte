<script>
  import { asset } from '$app/paths';
  import { onMount } from 'svelte';
  import { startChallengeTransition } from '$lib/stores/UiState.svelte';

  /**
   * @typedef {Object} Props
   * @property {number} index
   * @property {import('$lib/instructor/bouncy_instructor').ChallengeSet} set
   */

  /** @type {Props} */
  let { set, index } = $props();

  const href = `/challenges/${set.id}`;

  /** @type {HTMLAnchorElement | undefined} */
  let cardEl = $state();
  /** @type {HTMLDivElement | undefined} */
  let copyEl = $state();
  /** @type {HTMLHeadingElement | undefined} */
  let titleEl = $state();
  /** @type {HTMLDivElement | undefined} */
  let progressEl = $state();

  /** @type {HTMLDivElement | undefined} */
  let art = $state();
  /** @type {HTMLVideoElement | undefined} */
  let video = $state();
  let videoIndex = $state(0);
  let verticalCrop = $state(0);
  const videoChallenges = $derived(
    set.challenges.filter(
      (challenge) =>
        !challenge.solutionVideo.isEmpty() && challenge.solutionVideo.path()
    )
  );
  const activeVideo = $derived(videoChallenges[videoIndex]);

  /** @param {import('$lib/instructor/bouncy_instructor').Challenge} challenge */
  function previewStartTime(challenge) {
    const startMarker = challenge.solutionVideo.startMarkers()[0];
    const firstBeat = challenge.solutionVideo.beats()[0];
    return ((startMarker ?? firstBeat ?? 0) + 200) / 1000;
  }

  function updateCrop() {
    if (!art || !video || !video.videoWidth) return;
    const scale = Math.max(
      art.clientWidth / video.videoWidth,
      art.clientHeight / video.videoHeight
    );
    verticalCrop = Math.max(0, video.videoHeight * scale - art.clientHeight);
  }

  function playPreview() {
    if (!video || !activeVideo) return;
    updateCrop();
    video.currentTime = previewStartTime(activeVideo);
    video.play().catch(() => {});
  }

  function cycleVideo() {
    if (videoChallenges.length > 1) {
      videoIndex = (videoIndex + 1) % videoChallenges.length;
    }
  }

  onMount(() => {
    const resizeObserver = new ResizeObserver(updateCrop);
    if (art) resizeObserver.observe(art);
    const interval = videoChallenges.length
      ? window.setInterval(cycleVideo, 5000)
      : undefined;
    return () => {
      resizeObserver.disconnect();
      if (interval) window.clearInterval(interval);
    };
  });

  /** @param {MouseEvent} event */
  function handleClick(event) {
    if (
      event.defaultPrevented ||
      event.button !== 0 ||
      event.metaKey ||
      event.ctrlKey ||
      event.shiftKey ||
      event.altKey
    ) {
      return;
    }
    if (!cardEl || !copyEl || !titleEl || !progressEl) return;
    event.preventDefault();
    startChallengeTransition(href, set.id, {
      surfaceNode: copyEl,
      titleNode: titleEl,
      progressNode: progressEl,
    });
  }
</script>

<a class="set-card" {href} onclick={handleClick} bind:this={cardEl}>
  <div class="art" aria-hidden="true" bind:this={art}>
    {#if activeVideo}
      <video
        class="preview-video"
        class:panning={verticalCrop > 0}
        bind:this={video}
        src={activeVideo.solutionVideo.path()}
        muted
        loop
        playsinline
        onloadedmetadata={playPreview}
      ></video>
      <div class="video-tint"></div>
    {:else}
      <img class="dancer" src={asset('/icons/logo.svg')} alt="" />
    {/if}
    <span class="beat beat-one"></span>
    <span class="beat beat-two"></span>
    <span class="number">0{index + 1}</span>
  </div>
  <div class="card-copy" bind:this={copyEl}>
    <div class="label-row">
      <span>New</span>
      <span class="difficulty">Beginners</span>
    </div>
    <h2 bind:this={titleEl}>{set.name}</h2>
    <div class="reward-line">
      <span class="reward-label">Progress</span>
      <div class="reward-row" bind:this={progressEl}>
        0 / 1000
        <img
          src={asset('/img/symbols/bf_eye_black.svg')}
          alt=""
          class="currency"
        />
      </div>
    </div>
    <div class="card-bottom">
      <span><b>{set.challenges.length}</b> levels</span>
      <span class="start">Open Challenge <span aria-hidden="true">→</span></span
      >
    </div>
  </div>
</a>

<style>
  .set-card {
    display: grid;
    grid-template-columns: minmax(150px, 40%) 1fr;
    overflow: hidden;
    border-radius: 24px;
    color: var(--theme-neutral-white);
    background: var(--theme-neutral-black);
    transition:
      transform 180ms ease,
      box-shadow 180ms ease;
  }
  .set-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 20px 44px var(--theme-neutral-dark-transparent);
  }
  .art {
    position: relative;
    display: grid;
    place-items: center;
    min-height: 240px;
    overflow: hidden;
  }
  .preview-video {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
  }
  .preview-video.panning {
    animation: preview-pan 7.5s linear infinite;
  }
  .video-tint {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    background: var(--theme-main-dark);
    opacity: 0.72;
  }
  @keyframes preview-pan {
    from {
      object-position: center bottom;
    }
    to {
      object-position: center top;
    }
  }
  .art::before,
  .art::after {
    content: '';
    position: absolute;
    border: 2px solid var(--theme-neutral-dark-transparent);
    border-radius: 50%;
  }
  .art::before {
    width: 190px;
    height: 190px;
  }
  .art::after {
    width: 120px;
    height: 120px;
  }
  .dancer {
    position: relative;
    z-index: 1;
    width: var(--font-large);
    height: var(--font-large);
    transform: rotate(-8deg);
  }
  .number {
    position: absolute;
    bottom: 0.5rem;
    left: 0.7rem;
    color: var(--theme-neutral-white-60);
    font-size: var(--font-large);
    font-weight: 700;
  }
  .beat {
    position: absolute;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--theme-neutral-almost-black);
  }
  .beat-one {
    top: 20%;
    right: 17%;
  }
  .beat-two {
    right: 30%;
    bottom: 17%;
    width: 6px;
    height: 6px;
  }
  .card-copy {
    display: flex;
    flex-direction: column;
    padding: 1.25rem;
  }
  .label-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    color: var(--theme-main);
    /* font-size: var(--font-small); */
    font-weight: 700;
    letter-spacing: 0.09em;
    text-transform: uppercase;
  }
  .difficulty {
    padding: 0.3rem 0.5rem;
    border-radius: 99px;
    color: var(--theme-neutral-dark-gray);
    background: var(--theme-neutral-dark-transparent);
    letter-spacing: 0.02em;
    text-transform: none;
  }
  .card-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    margin-top: auto;
    padding-top: 1rem;
    color: var(--theme-neutral-dark-gray);
    /* font-size: var(--font-small); */
  }
  .card-bottom b {
    color: var(--theme-neutral-white);
  }
  .start {
    color: var(--theme-main);
  }
  .reward-line {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .reward-label {
    color: var(--theme-neutral-dark-gray);
  }
  .reward-row {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.35rem;
    padding: 0.3rem 0.65rem;
    border-radius: 999px;
    background: var(--theme-neutral-light);
    color: var(--theme-neutral-almost-black);
    font-size: var(--font-normal);
    font-weight: 800;
  }
  .currency {
    width: 1rem;
    height: 1rem;
  }

  @media (max-width: 600px) {
    .set-card {
      grid-template-columns: 1fr;
    }
    .art {
      min-height: 155px;
    }
    .card-copy {
      min-height: 190px;
    }
    .card-copy h2 {
      margin-top: 1.1rem;
    }
  }
</style>
