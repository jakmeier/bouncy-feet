<script>
  import { onMount } from 'svelte';
  import { parseChallengeString } from '$lib/instructor/bouncy_instructor';
  import { PoseDetection, landmarksToKeypoints } from '$lib/pose';

  /** @type {HTMLVideoElement} */
  let video = /** @type {HTMLVideoElement} */ (
    /** @type {unknown} */ (undefined)
  );
  let status = $state('Challenge keypoint export harness ready.');

  /**
   * Wait for one media event and turn media failures into useful tool errors.
   * @param {string} name
   * @returns {Promise<void>}
   */
  function mediaEvent(name) {
    return new Promise((resolve, reject) => {
      const cleanup = () => {
        video.removeEventListener(name, success);
        video.removeEventListener('error', failure);
      };
      const success = () => {
        cleanup();
        resolve();
      };
      const failure = () => {
        cleanup();
        reject(
          new Error(
            `video load failed: ${video.error?.message || 'unknown media error'}`
          )
        );
      };
      video.addEventListener(name, success, { once: true });
      video.addEventListener('error', failure, { once: true });
    });
  }

  /** @param {string} url */
  async function loadVideo(url) {
    video.crossOrigin = 'anonymous';
    video.src = url;
    video.load();
    if (video.readyState < HTMLMediaElement.HAVE_METADATA) {
      await mediaEvent('loadedmetadata');
    }
    if (!Number.isFinite(video.duration) || video.duration <= 0) {
      throw new Error(`video has an invalid duration: ${url}`);
    }
  }

  /** @param {number} timestampMs */
  async function frameAt(timestampMs) {
    const seconds = timestampMs / 1000;
    if (Math.abs(video.currentTime - seconds) > 0.0005) {
      const ready = mediaEvent('seeked');
      video.currentTime = seconds;
      await ready;
    }
    if (video.readyState < HTMLMediaElement.HAVE_CURRENT_DATA) {
      await mediaEvent('loadeddata');
    }
    // `seeked` identifies the decoded media frame; wait one paint before giving
    // that frame to MediaPipe. requestVideoFrameCallback can block on a paused
    // video when seeking to its existing timestamp, so it is not used here.
    await new Promise((resolve) =>
      requestAnimationFrame(() => resolve(undefined))
    );
  }

  /** @param {string} ron */
  function parse(ron) {
    return parseChallengeString(ron, 'en');
  }

  /** @param {string} ron */
  function descriptors(ron) {
    const set = parse(ron);
    return {
      setId: set.id,
      challenges: set.challenges.map((challenge) => ({
        id: challenge.id,
        videoUrl: challenge.solutionVideo.path(),
        startTimestamp: challenge.solutionStartTimestamp,
        startMarkers: challenge.solutionVideo.startMarkers(),
        beats: challenge.solutionVideo.beats(),
        bpm: challenge.bpm,
        parts: challenge.parts.map((part) => ({
          stepId: part.stepName,
          repeat: part.repeat,
          subbeatsPerMove: part.subbeatsPerMove,
        })),
      })),
    };
  }

  /**
   * Export [startMs, endMs], including both endpoints. With no explicit end,
   * the last configured beat is used; definitions with no useful beat range
   * use the end of the video. Frames are decoded only in increasing order.
   */
  /**
   * @param {string} ron
   * @param {string} challengeId
   * @param {{videoUrl?: string, startMs?: number, endMs?: number, frameMs?: number}} options
   */
  async function exportChallenge(ron, challengeId, options = {}) {
    const set = parse(ron);
    const challenges = set.challenges;
    const index = challenges.findIndex(
      (challenge) => challenge.id === challengeId
    );
    if (index < 0) throw new Error(`unknown challenge: ${challengeId}`);
    const challenge = challenges[index];
    const url = options.videoUrl || challenge.solutionVideo.path();
    status = `Loading ${challengeId}…`;
    await loadVideo(url);

    const configuredBeats = challenge.solutionVideo
      .beats()
      .filter((beat) => beat >= challenge.solutionStartTimestamp);
    const startMs = options.startMs ?? challenge.solutionStartTimestamp;
    const defaultEnd =
      configuredBeats.length > 1
        ? configuredBeats[configuredBeats.length - 1]
        : Math.floor(video.duration * 1000);
    const endMs = Math.min(
      options.endMs ?? defaultEnd,
      Math.floor(video.duration * 1000)
    );
    const frameMs = options.frameMs ?? 1000 / 30;
    if (!(startMs >= 0 && endMs > startMs && frameMs > 0)) {
      throw new Error(
        `invalid export range [${startMs}, ${endMs}] at ${frameMs} ms/frame`
      );
    }

    const tracker = set.tracker(index);
    if (!tracker) throw new Error(`challenge has no tracker: ${challengeId}`);
    let poses = 0;
    /** @type {(() => void) | undefined} */
    let callbackResolve;
    /** @type {((error: unknown) => void) | undefined} */
    let callbackReject;
    const detector = await PoseDetection.new((result, timestamp) => {
      try {
        if (timestamp === undefined)
          throw new Error('MediaPipe callback omitted its timestamp');
        if (result.landmarks?.[0]) {
          tracker.addKeypoints(
            landmarksToKeypoints(result.landmarks[0]),
            timestamp
          );
          poses += 1;
        }
        callbackResolve?.();
      } catch (error) {
        callbackReject?.(error);
      } finally {
        callbackResolve = undefined;
        callbackReject = undefined;
      }
    }, startMs);

    try {
      let previous = -Infinity;
      for (let timestamp = startMs; timestamp <= endMs; timestamp += frameMs) {
        const rounded = Math.min(endMs, Math.round(timestamp * 1000) / 1000);
        if (rounded <= previous) continue;
        previous = rounded;
        status = `Exporting ${challengeId}: ${Math.round(rounded)} / ${endMs} ms`;
        await frameAt(rounded);
        await new Promise((resolve, reject) => {
          callbackResolve = () => resolve(undefined);
          callbackReject = reject;
          detector.trackFrame(video, rounded);
        });
      }
    } finally {
      detector.close();
    }
    if (poses === 0) {
      throw new Error(
        `MediaPipe detected no person in ${challengeId} range [${startMs}, ${endMs}]`
      );
    }
    status = `Exported ${challengeId} (${poses} pose frames).`;
    return {
      ron: tracker.exportKeypoints(),
      startMs,
      endMs,
      frameCount: poses,
      videoUrl: url,
    };
  }

  onMount(() => {
    /** @type {any} */ (window).challengeKeypointExporter = {
      descriptors,
      exportChallenge,
    };
  });
</script>

<svelte:head><title>Challenge keypoint exporter</title></svelte:head>
<h1>Challenge keypoint exporter</h1>
<p>{status}</p>
<video bind:this={video} muted playsinline></video>

<style>
  video {
    max-width: 640px;
    max-height: 60vh;
  }
</style>
