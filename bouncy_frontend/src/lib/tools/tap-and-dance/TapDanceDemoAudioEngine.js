import { TapDanceTransport } from './TapDanceTransport.js';

/**
 * @typedef {'kick' | 'clap' | 'hat'} SoundId
 * @typedef {Object} DemoConfig
 * @property {number} bpm
 * @property {Record<SoundId, boolean>} enabled
 * @property {{startBpm: number, endBpm: number, duration: number, everyBeats: 4 | 8, stopAfterDuration?: boolean} | null} speedRamp
 */

/** @type {SoundId[]} */
const SOUND_IDS = ['kick', 'clap', 'hat'];

/** @extends {TapDanceTransport<DemoConfig>} */
export class TapDanceDemoAudioEngine extends TapDanceTransport {
  /**
   * @param {Record<SoundId, string>} samples
   * @param {() => DemoConfig} getConfig
   * @param {(step: number, bpm: number) => void} onStep
   * @param {() => void} [onComplete]
   */
  constructor(samples, getConfig, onStep, onComplete) {
    super(getConfig, onStep, onComplete);
    this.samples = samples;
    /** @type {Partial<Record<SoundId, AudioBuffer>>} */
    this.buffers = {};
    /** @type {Promise<void> | null} */
    this.loadPromise = null;
  }

  /** @param {DemoConfig} config @param {AudioContext} context */
  async loadAudio(config, context) {
    if (!this.loadPromise) {
      this.loadPromise = Promise.all(
        SOUND_IDS.map(async (id) => {
          const response = await fetch(this.samples[id]);
          if (!response.ok) throw new Error(`Could not load sample: ${id}`);
          this.buffers[id] = await context.decodeAudioData(
            await response.arrayBuffer()
          );
        })
      ).then(() => {});
    }
    try {
      await this.loadPromise;
    } catch (error) {
      this.loadPromise = null;
      throw error;
    }
  }

  /** @param {SoundId} id @param {number} time */
  playSample(id, time) {
    const buffer = this.buffers[id];
    if (!this.context || !buffer) return;
    const source = this.context.createBufferSource();
    source.buffer = buffer;
    source.connect(this.context.destination);
    // Play the entire original sample: no trimming, stretching or gain changes.
    source.start(time);
    if (Number.isFinite(this.exerciseEndTime))
      source.stop(this.exerciseEndTime);
    this.activeSources.add(source);
    source.onended = () => {
      this.activeSources.delete(source);
      source.disconnect();
    };
  }

  /** @param {number} step @param {number} time @param {DemoConfig} config */
  scheduleStep(step, time, config) {
    if (!this.context) return;
    if (config.enabled.kick && step % 2 === 0) this.playSample('kick', time);
    if (config.enabled.clap && step % 4 === 2) this.playSample('clap', time);
    if (config.enabled.hat && step % 2 === 1) this.playSample('hat', time);

    this.setTimeout(
      () => {
        if (this.running) this.onStep(step, config.bpm);
      },
      Math.max(0, (time - this.context.currentTime) * 1000)
    );
  }
}
