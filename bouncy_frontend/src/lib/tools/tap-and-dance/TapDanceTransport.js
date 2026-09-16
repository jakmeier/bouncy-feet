/**
 * Web Audio lifecycle and fixed eight-beat clock for the demo.
 *
 * @typedef {Object} PlaybackConfig
 * @property {number} bpm
 * @property {{startBpm: number, endBpm: number, duration: number, everyBeats: 4 | 8, stopAfterDuration?: boolean} | null} [speedRamp]
 */

/** @template {PlaybackConfig} Config */
export class TapDanceTransport {
  /**
   * @param {() => Config} getConfig
   * @param {(step: number, bpm: number) => void} onStep
   * @param {() => void} [onComplete]
   */
  constructor(getConfig, onStep, onComplete = () => {}) {
    this.getConfig = getConfig;
    this.onStep = onStep;
    this.onComplete = onComplete;
    this.exerciseEndTime = Infinity;
    /** @type {AudioContext | null} */
    this.context = null;
    /** @type {Set<AudioBufferSourceNode>} */
    this.activeSources = new Set();
    /** @type {OscillatorNode | null} */
    this.keepAliveOscillator = null;
    /** @type {GainNode | null} */
    this.keepAliveGain = null;
    /** @type {Set<number>} */
    this.timeouts = new Set();
    /** @type {number | undefined} */
    this.scheduler = undefined;
    this.currentStep = 0;
    this.exerciseStep = 0;
    this.exerciseStartTime = 0;
    this.playbackBpm = 0;
    /** @type {PlaybackConfig['speedRamp']} */
    this.speedRamp = null;
    this.nextStepTime = 0;
    this.running = false;
    this.disposed = false;
    this.playbackGeneration = 0;
  }

  /** @param {Config} config @param {AudioContext} context */
  // eslint-disable-next-line no-unused-vars -- implemented by the demo audio engine
  async loadAudio(config, context) {
    throw new Error('loadAudio must be implemented');
  }

  /** @param {number} step @param {number} time @param {Config} config */
  // eslint-disable-next-line no-unused-vars -- implemented by the demo audio engine
  scheduleStep(step, time, config) {
    throw new Error('scheduleStep must be implemented');
  }

  async ensureContext() {
    if (this.disposed) throw new Error('Audio engine is disposed');
    if (!this.context) this.context = new AudioContext();

    // These nodes are started synchronously from the button press, before the
    // first await. iOS can otherwise lose the user activation while samples
    // are fetched/decoded and leave the context suspended. The one-frame
    // buffer is the standard Web Audio unlock, while the silent oscillator
    // keeps Safari from suspending the context during preparation.
    const unlock = this.context.createBufferSource();
    unlock.buffer = this.context.createBuffer(1, 1, this.context.sampleRate);
    unlock.connect(this.context.destination);
    unlock.start();
    this.startKeepAlive();

    await this.context.resume();
    return this.context;
  }

  startKeepAlive() {
    if (!this.context || this.keepAliveOscillator) return;
    this.keepAliveOscillator = this.context.createOscillator();
    this.keepAliveGain = this.context.createGain();
    this.keepAliveGain.gain.value = 0;
    this.keepAliveOscillator
      .connect(this.keepAliveGain)
      .connect(this.context.destination);
    this.keepAliveOscillator.start();
  }

  stopKeepAlive() {
    if (this.keepAliveOscillator) {
      try {
        this.keepAliveOscillator.stop();
      } catch {
        // It may already have stopped while cleaning up the audio context.
      }
      this.keepAliveOscillator.disconnect();
    }
    this.keepAliveGain?.disconnect();
    this.keepAliveOscillator = null;
    this.keepAliveGain = null;
  }

  /** @param {() => void} callback @param {number} delay */
  setTimeout(callback, delay) {
    const timeout = window.setTimeout(() => {
      this.timeouts.delete(timeout);
      callback();
    }, delay);
    this.timeouts.add(timeout);
  }

  schedulerTick() {
    if (!this.context) return;
    if (this.context.currentTime >= this.exerciseEndTime) {
      this.stop();
      this.onComplete();
      return;
    }
    while (this.nextStepTime < this.context.currentTime + 0.1) {
      if (this.nextStepTime >= this.exerciseEndTime) break;
      const config = { ...this.getConfig() };
      if (this.speedRamp) {
        if (this.exerciseStep === 0) {
          this.exerciseStartTime = this.nextStepTime;
          if (this.speedRamp.stopAfterDuration) {
            this.exerciseEndTime =
              this.exerciseStartTime + this.speedRamp.duration;
            for (const source of this.activeSources)
              source.stop(this.exerciseEndTime);
          }
        }
        if (this.exerciseStep % (this.speedRamp.everyBeats * 2) === 0) {
          const progress = Math.min(
            1,
            Math.max(
              0,
              (this.nextStepTime - this.exerciseStartTime) /
                this.speedRamp.duration
            )
          );
          this.playbackBpm =
            this.speedRamp.startBpm +
            (this.speedRamp.endBpm - this.speedRamp.startBpm) * progress;
        }
        config.bpm = this.playbackBpm;
      }
      this.scheduleStep(this.currentStep, this.nextStepTime, config);
      this.exerciseStep += 1;
      this.nextStepTime += 30 / config.bpm;
      this.currentStep = (this.currentStep + 1) % 16;
    }
  }

  async start() {
    const generation = ++this.playbackGeneration;
    const context = await this.ensureContext();
    if (generation !== this.playbackGeneration || this.disposed) return false;
    const config = this.getConfig();
    await this.loadAudio(config, context);
    if (generation !== this.playbackGeneration || this.disposed) return false;
    // Freeze ramp settings for this run; restarting always begins at the start BPM.
    this.speedRamp = config.speedRamp ? { ...config.speedRamp } : null;
    this.playbackBpm = this.speedRamp?.startBpm ?? config.bpm;
    this.exerciseStep = 0;
    this.exerciseEndTime = Infinity;
    await context.resume();
    if (generation !== this.playbackGeneration || this.disposed) return false;
    if (context.state !== 'running') {
      this.stopKeepAlive();
      throw new Error(`Audio context is ${context.state}`);
    }
    this.running = true;
    this.currentStep = 0;
    this.onStep(-1, this.playbackBpm);
    this.nextStepTime = context.currentTime + 0.06;
    this.schedulerTick();
    this.scheduler = window.setInterval(() => this.schedulerTick(), 25);
    return true;
  }

  stop() {
    // Cancel pending async starts as well as already scheduled audio.
    this.playbackGeneration += 1;
    this.running = false;
    this.exerciseEndTime = Infinity;
    if (this.scheduler !== undefined) window.clearInterval(this.scheduler);
    this.scheduler = undefined;
    for (const timeout of this.timeouts) window.clearTimeout(timeout);
    this.timeouts.clear();
    for (const source of this.activeSources) {
      try {
        source.stop();
      } catch {
        // The source may have ended between scheduling and cleanup.
      }
    }
    this.activeSources.clear();
    this.stopKeepAlive();
  }

  async dispose() {
    this.disposed = true;
    this.stop();
    await this.context?.close();
  }
}
